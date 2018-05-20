// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait ShortyService {
    fn shorten(&self, o: ::grpc::RequestOptions, p: super::shorty::ShortenRequest) -> ::grpc::SingleResponse<super::shorty::Shorty>;

    fn add_counter(&self, o: ::grpc::RequestOptions, p: super::shorty::Counter) -> ::grpc::SingleResponse<super::shorty::Empty>;

    fn get_url(&self, o: ::grpc::RequestOptions, p: super::shorty::Shorty) -> ::grpc::SingleResponse<super::shorty::Shorty>;
}

// client

pub struct ShortyServiceClient {
    grpc_client: ::grpc::Client,
    method_Shorten: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::shorty::ShortenRequest, super::shorty::Shorty>>,
    method_AddCounter: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::shorty::Counter, super::shorty::Empty>>,
    method_GetUrl: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::shorty::Shorty, super::shorty::Shorty>>,
}

impl ShortyServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ShortyServiceClient {
            grpc_client: grpc_client,
            method_Shorten: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/shorty.ShortyService/Shorten".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AddCounter: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/shorty.ShortyService/AddCounter".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetUrl: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/shorty.ShortyService/GetUrl".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ShortyServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ShortyServiceClient::with_client(c)
        })
    }
}

impl ShortyService for ShortyServiceClient {
    fn shorten(&self, o: ::grpc::RequestOptions, p: super::shorty::ShortenRequest) -> ::grpc::SingleResponse<super::shorty::Shorty> {
        self.grpc_client.call_unary(o, p, self.method_Shorten.clone())
    }

    fn add_counter(&self, o: ::grpc::RequestOptions, p: super::shorty::Counter) -> ::grpc::SingleResponse<super::shorty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_AddCounter.clone())
    }

    fn get_url(&self, o: ::grpc::RequestOptions, p: super::shorty::Shorty) -> ::grpc::SingleResponse<super::shorty::Shorty> {
        self.grpc_client.call_unary(o, p, self.method_GetUrl.clone())
    }
}

// server

pub struct ShortyServiceServer;


impl ShortyServiceServer {
    pub fn new_service_def<H : ShortyService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/shorty.ShortyService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/shorty.ShortyService/Shorten".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.shorten(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/shorty.ShortyService/AddCounter".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_counter(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/shorty.ShortyService/GetUrl".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_url(o, p))
                    },
                ),
            ],
        )
    }
}
