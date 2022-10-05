extern crate serde;
extern crate serde_derive;
use futures::TryFutureExt;
use jsonpath::Selector;
use log::{debug, error, info};
use reqwest::Url;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug)]
enum Error {
	Operation,
	CannotGetWeb,
	CannotGetWebContent,
	FailedToParseJson,
	NotSupportRust,
	CreateFileFailed,
	ParseDataFailed,
	ConvertTypeFailed,
	QuestionIdNotFound,
}

#[derive(StructOpt)]
#[structopt(name = "leetcode-rust", about = "Create leetcode template")]
struct Opt {
	#[structopt(help = "The URL to a leetcode question")]
	url: Url,
}

fn initial_log() {
	pretty_env_logger::formatted_builder()
		.filter_level(log::LevelFilter::Info)
		.init();
}

fn get_args() -> Result<Opt, Error> {
	let opt = Opt::from_args();
	if opt.url.host_str().unwrap() != "leetcode.com" {
		error!("Incorrect host: {:?}", opt.url.host_str());
		Err(Error::Operation)
	} else {
		Ok(opt)
	}
}

async fn get_data_from_leetcode(question_name: &str) -> Result<Value, Error> {
	// 不清楚應該是要用 questionId 還是 questionFrontendId
	let req = json!({
		"operationName": "questionData",
		"variables": { "titleSlug": &question_name },
		"query": r#"
	        query questionData($titleSlug: String) {
	            question(titleSlug: $titleSlug) {
                    questionId
	                questionFrontendId
	                codeSnippets {
	                    lang
	                    langSlug
	                    code
	                    __typename
	                }
	            }
	        }
	    "#
	});
	let client = reqwest::Client::new();
	let resp = client
		.post("https://leetcode.com/graphql")
		.header("ACCEPT", "*/*")
		.header("CONTENT-TYPE", "application/json")
		.body(req.to_string())
		.send()
		.map_err(|_| Error::CannotGetWeb)
		.await?;

	let body = resp.text().await.map_err(|_| Error::CannotGetWebContent)?;
	serde_json::from_str::<Value>(&body).or(Err(Error::FailedToParseJson))
}

fn get_code(json_str: &Value) -> Result<&str, Error> {
	let selector =
		Selector::new("$.data.question.codeSnippets.*").or(Err(Error::ParseDataFailed))?;
	let snippets = selector
		.find(json_str)
		.map(|t| (t["lang"].as_str().unwrap(), t["code"].as_str().unwrap()))
		.collect::<HashMap<&str, &str>>();
	snippets
		.into_iter()
		.find(|(k, _)| *k == "Rust")
		.map(|(_, code)| code)
		.ok_or(Error::NotSupportRust)
}

fn get_question_id(json_str: &Value) -> Result<u16, Error> {
	let selector =
		Selector::new("$.data.question.questionFrontendId").or(Err(Error::ParseDataFailed))?;
	selector
		.find(&json_str)
		.map(|t| {
			t.as_str()
				.map(|v| {
					debug!("question id str: {v}");
					v.parse::<u16>().or(Err(Error::ConvertTypeFailed))
				})
				.ok_or(Error::ConvertTypeFailed)
		})
		.last()
		.ok_or(Error::QuestionIdNotFound)??
}

fn mix_code_with_template(code: &str) -> String {
	format!(
		r#"use crate::solutions::Solution;


{code}


#[cfg(test)]
mod tests{{
    use test_case::test_case;

    fn cases() {{
    }}
}}
"#
	)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	initial_log();

	info!("start...");
	let opt = get_args()?;
	let question_name = opt.url.path().replace("/problems/", "").replace("/", "");
	info!("question name: {question_name}");

	info!("get from leetcode...");
	let value = get_data_from_leetcode(&question_name).await?;
	debug!("value: {:?}", value);

	let code = mix_code_with_template(get_code(&value)?);
	debug!("code: {code}");
	let question_id = get_question_id(&value)?;
	debug!("question id: {code}");
	let format_question_name = question_name.replace('-', "_");
	debug!("formatted question: {code}");

	info!("create file...");
	let current_dir = env::current_dir().unwrap();
	let file_name = format!("s{:0>4}_{}.rs", question_id, format_question_name);
	let file_path = PathBuf::from(current_dir).join(format!("src/solutions/{file_name}"));
	let mut file = File::create(file_path).map_err(|_| Error::CreateFileFailed)?;
	file.write_all(code.as_bytes())
		.or(Err(Error::CreateFileFailed))?;

	info!("Done");
	Ok(())
}
