use google_youtube3::Error;
use google_youtube3::api::CommentThreadListResponse;
use google_youtube3::yup_oauth2::ApplicationSecret;
use google_youtube3::{YouTube, hyper_rustls, hyper_util, yup_oauth2};
use yup_oauth2::authenticator_delegate::InstalledFlowDelegate;

struct BrowserDelegate;

impl InstalledFlowDelegate for BrowserDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        _need_code: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send + 'a>>
    {
        Box::pin(async move {
            open::that(url).map_err(|e| e.to_string())?;
            Ok(String::new())
        })
    }
}

fn build_secret() -> ApplicationSecret {
    let secret = yup_oauth2::ApplicationSecret {
        client_id: std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set"),
        client_secret: std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET not set"),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".into(),
        token_uri: "https://oauth2.googleapis.com/token".into(),
        redirect_uris: vec!["http://localhost".into()],
        project_id: None,
        client_email: None,
        auth_provider_x509_cert_url: None,
        client_x509_cert_url: None,
    };
    return secret;
}

fn search_comments(response: &CommentThreadListResponse, query: String) {
    let vec_comment_threads = &response.items;
    for vec in vec_comment_threads.iter() {
        for entry in vec {
            for snippet in entry.snippet.iter() {
                if let Some(comment) = &snippet
                    .top_level_comment
                    .as_ref()
                    .and_then(|c| c.snippet.as_ref())
                    .and_then(|s| s.text_original.as_ref())
                {
                    if comment.contains(&query) {
                        println!(
                            "Your string was found in the following comment: {}",
                            comment
                        )
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let secret = build_secret();
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_only()
        .enable_http2()
        .build();

    let executor = hyper_util::rt::TokioExecutor::new();
    let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        yup_oauth2::client::CustomHyperClientBuilder::from(
            hyper_util::client::legacy::Client::builder(executor).build(connector),
        ),
    )
    .flow_delegate(Box::new(BrowserDelegate))
    .build()
    .await
    .unwrap();

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http2()
                .build(),
        );
    let hub = YouTube::new(client, auth);
    let result = hub
        .comment_threads()
        .list(&vec!["snippet".into()])
        .video_id("G8NWijCUcXg")
        .doit()
        .await;

    match result {
        Err(e) => match e {
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => {
            let search_str = std::env::var("SEARCH_STRING").expect("SEARCH_STRING not set");
            search_comments(&res.1, search_str);
        }
    }
}
