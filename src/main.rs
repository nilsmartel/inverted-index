use std::collections::HashMap;
use word_iter::Words;

type Term = String;
type Document = String;

#[derive(Default)]
struct InformationRetrieval {
    inverted_index: HashMap<Term, Vec<Document>>,
}

fn main() {
    let mut ir = InformationRetrieval::default();

    for doc in std::env::args().skip(1) {
        let content = std::fs::read_to_string(&doc).expect("to read document");

        let terms = content.words();

        for term in terms {
            let index = ir.inverted_index.entry(term.to_string()).or_default();

            index.push(doc.to_string());
        }
    }

    for term in ir.inverted_index.keys() {
        let doc = ir.inverted_index.get(term);
        println!("{term}:   {doc:?}");
    }
}
