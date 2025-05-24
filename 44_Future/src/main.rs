use trpl::Html;
async fn page_title(url:&str)->Option<String>{
    let response=trpl::get(url).await;
    let text=response.text().await;
    Html::parse(&text)
        .select_first("title")
        .map(|title|{
            title.inner_html()
        })
}
fn main() {
    let url="https://www.rust-lang.org/";
    trpl::run(async {
        let title=page_title(url).await;
        match title{
            Some(title)=>{
                println!("Title: {}",title);
            }
            None=>{
                println!("No title found");
            }
        }
    })
}