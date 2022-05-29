use tantivy::IndexReader;
use tantivy::IndexWriter;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
use tempfile::TempDir;
use anyhow::Result;
use async_std::sync::Mutex;
use std::sync::Arc;

// #[derive(Clone)]
struct SchemaHandler {
    schema: Schema,
    title: Field,
    body: Field
}

impl SchemaHandler {
    fn new() -> Self {
        let schema = get_schema().unwrap();
        let title = schema.get_field("title").unwrap();
        let body = schema.get_field("body").unwrap();
        SchemaHandler { schema, title, body }
    }
}

fn get_schema() -> Result<Schema, anyhow::Error> {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();

    Ok(schema)
}

// #[derive(Clone)]
pub struct SearchHandler {
    schema: SchemaHandler,
    index: Index,
    index_writer: Arc<Mutex<IndexWriter>>,
    reader: IndexReader
}

impl SearchHandler {
    pub fn new() -> Self {
        let schema = SchemaHandler::new();
        let index_path = TempDir::new().unwrap();
        let index = Index::create_in_dir("/home/sankar/lily_data", schema.schema.clone()).unwrap();
        let index_writer = index.writer(50_000_000).unwrap();
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into().unwrap();
    
        SearchHandler {
            schema,
            index,
            index_writer: Arc::new(Mutex::new(index_writer)),
            reader
        }
    }

    pub fn create_document(&mut self, title: &str, body: &str) {
        let mut old_man_doc = Document::default();
        old_man_doc.add_text(self.schema.title, title);
        old_man_doc.add_text(
            self.schema.body,
            body,
        );
        let a = Arc::clone(&self.index_writer);
        let b = a.as_ref();
        // a.index_writer.get_mut().add_document(old_man_doc).unwrap();
        // self.index_writer.get_mut().commit().unwrap();
    }

    pub fn search(&self, query: &str) -> Result<(), anyhow::Error> {
        let searcher = self.reader.searcher();
        let query_parser = QueryParser::for_index(&self.index, vec![self.schema.title, self.schema.body]);
        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            println!("{}", self.schema.schema.to_json(&retrieved_doc));
        }
        Ok(())
    }

}

// fn create_document_two(schema: &Schema, index_writer: &mut IndexWriter) -> Result<(), anyhow::Error> {
//     let title = schema.get_field("title").unwrap();
//     let body = schema.get_field("body").unwrap();
//     let mut old_man_doc = Document::default();
//     index_writer.add_document(doc!(
//         title => "Of Mice and Men",
//         body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
//                 bank and runs deep and green. The water is warm too, for it has slipped twinkling \
//                 over the yellow sands in the sunlight before reaching the narrow pool. On one \
//                 side of the river the golden foothill slopes curve up to the strong and rocky \
//                 Gabilan Mountains, but on the valley side the water is lined with trees—willows \
//                 fresh and green with every spring, carrying in their lower leaf junctures the \
//                 debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
//                 limbs and branches that arch over the pool"
//         ));
//     index_writer.add_document(doc!(
//     title => "Of Mice and Men",
//     body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
//             bank and runs deep and green. The water is warm too, for it has slipped twinkling \
//             over the yellow sands in the sunlight before reaching the narrow pool. On one \
//             side of the river the golden foothill slopes curve up to the strong and rocky \
//             Gabilan Mountains, but on the valley side the water is lined with trees—willows \
//             fresh and green with every spring, carrying in their lower leaf junctures the \
//             debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
//             limbs and branches that arch over the pool"
//     ));

//     index_writer.add_document(doc!(
//         title => "Frankenstein",
//         title => "The Modern Prometheus",
//         body => "You will rejoice to hear that no disaster has accompanied the commencement of an \
//                  enterprise which you have regarded with such evil forebodings.  I arrived here \
//                  yesterday, and my first task is to assure my dear sister of my welfare and \
//                  increasing confidence in the success of my undertaking."
//         ));
    
//     index_writer.commit()?;
//     Ok(())
    
// }
