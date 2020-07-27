// Copyright (c) Microsoft. All rights reserved.

#[allow(clippy::needless_pass_by_value)] // TODO: Remove when the stub is filled out and `inner` actually gets used.
pub(super) fn handle(
    req: hyper::Request<hyper::Body>,
    inner: std::sync::Arc<aziot_identityd::Server>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<hyper::Response<hyper::Body>, hyper::Request<hyper::Body>>> + Send>> {
    Box::pin(async move {
        if req.uri().path() != "/trust-bundle" {
            return Err(req);
        }

        let user = aziot_identityd::auth::Uid(0);
        let auth_id = match inner.authenticator.authenticate(user) {
            Ok(auth_id) => auth_id,
            Err(err) => return Ok(super::ToHttpResponse::to_http_response(&err)),
        };

        let (http::request::Parts { method, .. }, _body) = req.into_parts();

        if method != hyper::Method::GET {
            return Ok(super::err_response(
                hyper::StatusCode::METHOD_NOT_ALLOWED,
                Some((hyper::header::ALLOW, "GET")),
                "method not allowed".into(),
            ));
        }

        //TODO: get uid from UDS
        let response = match inner.get_trust_bundle(auth_id) {
            Ok(v) => v,
            Err(err) => return Ok(super::ToHttpResponse::to_http_response(&err)),
        };

        let response = aziot_identity_common_http::get_trust_bundle::Response {
            certificate: response
        };

        let response = super::json_response(hyper::StatusCode::OK, &response);
        Ok(response)

        }
    )
}
