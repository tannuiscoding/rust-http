use std::{collections::HashMap, sync::Arc};

use crate::{request::{Method, Request}, response::Response};

type Handler = Arc<dyn Fn(&Request) -> Response + Send + Sync>;

#[derive(Default, Clone)]
pub struct Router {
    routes: HashMap<String, HashMap<Method, Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_route<F>(&mut self, method: Method, path: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes
            .entry(path.to_string())
            .or_default()
            .insert(method, Arc::new(handler));
    }

    pub fn route(&self, request: &mut Request) -> Response {
        let path = request.path_without_query();

        for (pattern, handlers) in &self.routes {
            if let Some(params) = Self::match_pattern(pattern, path) {
                request.params = params;
                if let Some(handler) = handlers.get(&request.method) {
                    return handler(request);
                }
                return Response::new(405, "");
            }
        }

        Response::new(404, "")
    }

    fn match_pattern(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
        let pattern_segments: Vec<&str> = pattern
            .trim_matches('/')
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();
        let path_segments: Vec<&str> = path
            .trim_matches('/')
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();

        if pattern_segments.len() != path_segments.len() {
            return None;
        }

        let mut params = HashMap::new();
        for (pattern_segment, path_segment) in pattern_segments.iter().zip(path_segments.iter()) {
            if pattern_segment.starts_with(':') {
                params.insert(pattern_segment[1..].to_string(), (*path_segment).to_string());
            } else if pattern_segment != path_segment {
                return None;
            }
        }

        Some(params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_route_params() {
        let mut router = Router::new();
        router.add_route(Method::Get, "/users/:id", |request: &Request| {
            let id = request
                .params
                .get("id")
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            Response::text(200, format!("user {id}"))
        });

        let mut request = Request {
            method: Method::Get,
            path: "/users/42".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: String::new(),
            params: HashMap::new(),
        };

        let response = router.route(&mut request);
        assert_eq!(response.status_code, 200);
        assert_eq!(response.body, "user 42");
    }

    #[test]
    fn returns_method_not_allowed_for_existing_path() {
        let mut router = Router::new();
        router.add_route(Method::Get, "/", |_| Response::text(200, "ok"));

        let mut request = Request {
            method: Method::Post,
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: String::new(),
            params: HashMap::new(),
        };

        let response = router.route(&mut request);
        assert_eq!(response.status_code, 405);
    }
}
