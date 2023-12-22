use log::info;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use tracing::{info_span, Span};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RequestId<T = String>(pub T);

#[derive(Clone)]
pub struct TracingSpan<T = Span>(T);

pub struct TracingFairing;

/// Fairing for tracing.
/// It is configured to add tracing to requests and responses.
/// It also adds a request id to the response.
/// It is configured to use OpenTelemetry.
#[rocket::async_trait]
impl Fairing for TracingFairing {
    fn info(&self) -> Info {
        Info {
            name: "Tracing Fairing",
            kind: Kind::Request | Kind::Response,
        }
    }
    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let user_agent = req.headers().get_one("User-Agent").unwrap_or("");
        let request_id = req
            .headers()
            .get_one("X-Request-Id")
            .map(ToString::to_string)
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        req.local_cache(|| RequestId(Some(request_id.to_owned())));

        let span = info_span!(
            "request",
            otel.name=%format!("{} {}", req.method(), req.uri().path()),
            http.method = %req.method(),
            http.uri = %req.uri().path(),
            http.user_agent=%user_agent,
            http.status_code = tracing::field::Empty,
            http.request_id=%request_id,
        );

        req.local_cache(|| TracingSpan::<Option<Span>>(Some(span)));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        if let Some(span) = req
            .local_cache(|| TracingSpan::<Option<Span>>(None))
            .0
            .to_owned()
        {
            let _entered_span = span.entered();
            _entered_span.record("http.status_code", res.status().code);

            if let Some(request_id) = &req.local_cache(|| RequestId::<Option<String>>(None)).0 {
                info!(
                    "Returning request with id {} with {}",
                    request_id,
                    res.status()
                );
            }

            drop(_entered_span);
        }

        if let Some(request_id) = &req.local_cache(|| RequestId::<Option<String>>(None)).0 {
            res.set_raw_header("X-Request-Id", request_id);
        }
    }
}
