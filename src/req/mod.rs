use std::collections::HashMap;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use chrono::Utc;
use futures::StreamExt;

use crate::data::{PushData, PushList, PushRule, RuleData};


pub async fn create_push_rule(params: web::Query<PushRule>,
                              rule_data: web::Data<RuleData>) -> String {
    let now = Utc::now().timestamp();
    let rule = params.0;
    let mut value = rule_data.data.write().unwrap();
    value.insert(now, rule);
    let addr = String::from("http://127.0.0.1:8080/mock/");
    addr + &now.to_string()
}

pub async fn accept_push(req: HttpRequest,
                         push_id: web::Path<(i64, )>,
                         rule_data: web::Data<RuleData>,
                         mut payload: web::Payload,
                         data: web::Data<PushList>) -> HttpResponse {
    let rule = rule_data.data.read().unwrap();

    if let Some(rule) = rule.get(&push_id.0) {
        let params = req.query_string();

        let mut bytes = web::BytesMut::new();
        while let Some(item) = payload.next().await {
            let item = item.unwrap();
            bytes.extend_from_slice(&item);
        }
        let content: String = std::str::from_utf8(&bytes).unwrap().into();

        let mut header_map = HashMap::new();
        for (name, value) in req.headers().iter() {
            header_map.insert(String::from(name.as_str()), String::from(value.to_str().unwrap()));
        }

        let push_data = PushData {
            raw_body: params.into(),
            push_header: header_map,
            body: Some(content),
        };

        let mut container = data.data.write().unwrap();
        let rt = container.get(&push_id.0);
        if rt.is_none() {
            let new: Vec<PushData> = Vec::new();
            container.insert(push_id.0, new);
        }
        let list = container.get_mut(&push_id.0).unwrap();
        list.push(push_data);

        let status_code = StatusCode::from_u16(rule.status_code as u16).unwrap();
        return HttpResponse::build(status_code)
            .body(&rule.response);
    }

    HttpResponse::Ok().finish()
}

pub async fn push_record(push_id: web::Path<(i64, )>,
                         data: web::Data<PushList>) -> HttpResponse {
    let container = data.data.read().unwrap();
    let rt = container.get(&push_id.0);
    let mut builder = HttpResponse::Ok();
    if rt.is_none() {
        let empty: Vec<PushData> = Vec::new();
        return builder.json(empty);
    }
    builder.json2(rt.unwrap())
}


