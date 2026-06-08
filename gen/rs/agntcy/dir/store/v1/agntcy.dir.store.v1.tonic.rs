// @generated
/// Generated client implementations.
pub mod store_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StoreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StoreServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StoreServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StoreServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            StoreServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn push(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::core::v1::Record,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::core::v1::RecordRef>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.StoreService/Push",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("agntcy.dir.store.v1.StoreService", "Push"));
            self.inner.streaming(req, path, codec).await
        }
        pub async fn pull(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::core::v1::RecordRef,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::core::v1::Record>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.StoreService/Pull",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("agntcy.dir.store.v1.StoreService", "Pull"));
            self.inner.streaming(req, path, codec).await
        }
        pub async fn lookup(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::core::v1::RecordRef,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::core::v1::RecordMeta>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.StoreService/Lookup",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("agntcy.dir.store.v1.StoreService", "Lookup"));
            self.inner.streaming(req, path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::core::v1::RecordRef,
            >,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.StoreService/Delete",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("agntcy.dir.store.v1.StoreService", "Delete"));
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn push_referrer(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::PushReferrerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PushReferrerResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.StoreService/PushReferrer",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("agntcy.dir.store.v1.StoreService", "PushReferrer"),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn pull_referrer(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::PullReferrerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PullReferrerResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.StoreService/PullReferrer",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("agntcy.dir.store.v1.StoreService", "PullReferrer"),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn delete_referrer(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::DeleteReferrerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DeleteReferrerResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.StoreService/DeleteReferrer",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("agntcy.dir.store.v1.StoreService", "DeleteReferrer"),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod store_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StoreServiceServer.
    #[async_trait]
    pub trait StoreService: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the Push method.
        type PushStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::core::v1::RecordRef,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn push(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::core::v1::Record>,
            >,
        ) -> std::result::Result<tonic::Response<Self::PushStream>, tonic::Status>;
        /// Server streaming response type for the Pull method.
        type PullStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::core::v1::Record,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn pull(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::core::v1::RecordRef>,
            >,
        ) -> std::result::Result<tonic::Response<Self::PullStream>, tonic::Status>;
        /// Server streaming response type for the Lookup method.
        type LookupStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::super::super::core::v1::RecordMeta,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn lookup(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::core::v1::RecordRef>,
            >,
        ) -> std::result::Result<tonic::Response<Self::LookupStream>, tonic::Status>;
        async fn delete(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::core::v1::RecordRef>,
            >,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Server streaming response type for the PushReferrer method.
        type PushReferrerStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::PushReferrerResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn push_referrer(
            &self,
            request: tonic::Request<tonic::Streaming<super::PushReferrerRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::PushReferrerStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the PullReferrer method.
        type PullReferrerStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::PullReferrerResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn pull_referrer(
            &self,
            request: tonic::Request<tonic::Streaming<super::PullReferrerRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::PullReferrerStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DeleteReferrer method.
        type DeleteReferrerStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DeleteReferrerResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn delete_referrer(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeleteReferrerRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::DeleteReferrerStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct StoreServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> StoreServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StoreServiceServer<T>
    where
        T: StoreService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/agntcy.dir.store.v1.StoreService/Push" => {
                    #[allow(non_camel_case_types)]
                    struct PushSvc<T: StoreService>(pub Arc<T>);
                    impl<
                        T: StoreService,
                    > tonic::server::StreamingService<
                        super::super::super::core::v1::Record,
                    > for PushSvc<T> {
                        type Response = super::super::super::core::v1::RecordRef;
                        type ResponseStream = T::PushStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::core::v1::Record>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StoreService>::push(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PushSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.StoreService/Pull" => {
                    #[allow(non_camel_case_types)]
                    struct PullSvc<T: StoreService>(pub Arc<T>);
                    impl<
                        T: StoreService,
                    > tonic::server::StreamingService<
                        super::super::super::core::v1::RecordRef,
                    > for PullSvc<T> {
                        type Response = super::super::super::core::v1::Record;
                        type ResponseStream = T::PullStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::core::v1::RecordRef>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StoreService>::pull(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PullSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.StoreService/Lookup" => {
                    #[allow(non_camel_case_types)]
                    struct LookupSvc<T: StoreService>(pub Arc<T>);
                    impl<
                        T: StoreService,
                    > tonic::server::StreamingService<
                        super::super::super::core::v1::RecordRef,
                    > for LookupSvc<T> {
                        type Response = super::super::super::core::v1::RecordMeta;
                        type ResponseStream = T::LookupStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::core::v1::RecordRef>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StoreService>::lookup(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LookupSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.StoreService/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: StoreService>(pub Arc<T>);
                    impl<
                        T: StoreService,
                    > tonic::server::ClientStreamingService<
                        super::super::super::core::v1::RecordRef,
                    > for DeleteSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::core::v1::RecordRef>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StoreService>::delete(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.StoreService/PushReferrer" => {
                    #[allow(non_camel_case_types)]
                    struct PushReferrerSvc<T: StoreService>(pub Arc<T>);
                    impl<
                        T: StoreService,
                    > tonic::server::StreamingService<super::PushReferrerRequest>
                    for PushReferrerSvc<T> {
                        type Response = super::PushReferrerResponse;
                        type ResponseStream = T::PushReferrerStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::PushReferrerRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StoreService>::push_referrer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PushReferrerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.StoreService/PullReferrer" => {
                    #[allow(non_camel_case_types)]
                    struct PullReferrerSvc<T: StoreService>(pub Arc<T>);
                    impl<
                        T: StoreService,
                    > tonic::server::StreamingService<super::PullReferrerRequest>
                    for PullReferrerSvc<T> {
                        type Response = super::PullReferrerResponse;
                        type ResponseStream = T::PullReferrerStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::PullReferrerRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StoreService>::pull_referrer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PullReferrerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.StoreService/DeleteReferrer" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteReferrerSvc<T: StoreService>(pub Arc<T>);
                    impl<
                        T: StoreService,
                    > tonic::server::StreamingService<super::DeleteReferrerRequest>
                    for DeleteReferrerSvc<T> {
                        type Response = super::DeleteReferrerResponse;
                        type ResponseStream = T::DeleteReferrerStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::DeleteReferrerRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StoreService>::delete_referrer(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteReferrerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for StoreServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "agntcy.dir.store.v1.StoreService";
    impl<T> tonic::server::NamedService for StoreServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod sync_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SyncServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SyncServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SyncServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SyncServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            SyncServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.SyncService/CreateSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("agntcy.dir.store.v1.SyncService", "CreateSync"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_syncs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSyncsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ListSyncsItem>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.SyncService/ListSyncs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("agntcy.dir.store.v1.SyncService", "ListSyncs"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.SyncService/GetSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("agntcy.dir.store.v1.SyncService", "GetSync"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.SyncService/DeleteSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("agntcy.dir.store.v1.SyncService", "DeleteSync"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_registry_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestRegistryCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RequestRegistryCredentialsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agntcy.dir.store.v1.SyncService/RequestRegistryCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "agntcy.dir.store.v1.SyncService",
                        "RequestRegistryCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod sync_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SyncServiceServer.
    #[async_trait]
    pub trait SyncService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_sync(
            &self,
            request: tonic::Request<super::CreateSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSyncResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListSyncs method.
        type ListSyncsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ListSyncsItem, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_syncs(
            &self,
            request: tonic::Request<super::ListSyncsRequest>,
        ) -> std::result::Result<tonic::Response<Self::ListSyncsStream>, tonic::Status>;
        async fn get_sync(
            &self,
            request: tonic::Request<super::GetSyncRequest>,
        ) -> std::result::Result<tonic::Response<super::GetSyncResponse>, tonic::Status>;
        async fn delete_sync(
            &self,
            request: tonic::Request<super::DeleteSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteSyncResponse>,
            tonic::Status,
        >;
        async fn request_registry_credentials(
            &self,
            request: tonic::Request<super::RequestRegistryCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RequestRegistryCredentialsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SyncServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> SyncServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SyncServiceServer<T>
    where
        T: SyncService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/agntcy.dir.store.v1.SyncService/CreateSync" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSyncSvc<T: SyncService>(pub Arc<T>);
                    impl<
                        T: SyncService,
                    > tonic::server::UnaryService<super::CreateSyncRequest>
                    for CreateSyncSvc<T> {
                        type Response = super::CreateSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSyncRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SyncService>::create_sync(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.SyncService/ListSyncs" => {
                    #[allow(non_camel_case_types)]
                    struct ListSyncsSvc<T: SyncService>(pub Arc<T>);
                    impl<
                        T: SyncService,
                    > tonic::server::ServerStreamingService<super::ListSyncsRequest>
                    for ListSyncsSvc<T> {
                        type Response = super::ListSyncsItem;
                        type ResponseStream = T::ListSyncsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSyncsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SyncService>::list_syncs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListSyncsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.SyncService/GetSync" => {
                    #[allow(non_camel_case_types)]
                    struct GetSyncSvc<T: SyncService>(pub Arc<T>);
                    impl<
                        T: SyncService,
                    > tonic::server::UnaryService<super::GetSyncRequest>
                    for GetSyncSvc<T> {
                        type Response = super::GetSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSyncRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SyncService>::get_sync(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.SyncService/DeleteSync" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSyncSvc<T: SyncService>(pub Arc<T>);
                    impl<
                        T: SyncService,
                    > tonic::server::UnaryService<super::DeleteSyncRequest>
                    for DeleteSyncSvc<T> {
                        type Response = super::DeleteSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteSyncRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SyncService>::delete_sync(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/agntcy.dir.store.v1.SyncService/RequestRegistryCredentials" => {
                    #[allow(non_camel_case_types)]
                    struct RequestRegistryCredentialsSvc<T: SyncService>(pub Arc<T>);
                    impl<
                        T: SyncService,
                    > tonic::server::UnaryService<
                        super::RequestRegistryCredentialsRequest,
                    > for RequestRegistryCredentialsSvc<T> {
                        type Response = super::RequestRegistryCredentialsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RequestRegistryCredentialsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SyncService>::request_registry_credentials(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RequestRegistryCredentialsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for SyncServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "agntcy.dir.store.v1.SyncService";
    impl<T> tonic::server::NamedService for SyncServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
