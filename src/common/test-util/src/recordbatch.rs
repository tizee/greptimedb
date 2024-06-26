// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_query::OutputData;
use common_recordbatch::util;

pub enum ExpectedOutput<'a> {
    AffectedRows(usize),
    QueryResult(&'a str),
}

pub async fn check_output_stream(output: OutputData, expected: &str) {
    let recordbatches = match output {
        OutputData::Stream(stream) => util::collect_batches(stream).await.unwrap(),
        OutputData::RecordBatches(recordbatches) => recordbatches,
        _ => unreachable!(),
    };
    let pretty_print = recordbatches.pretty_print().unwrap();
    assert_eq!(pretty_print, expected, "actual: \n{}", pretty_print);
}
