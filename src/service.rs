use super::MapperMiddleware;
use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, ResponseError,
};
use futures_util::future;
use std::{marker::PhantomData, rc::Rc};

/// # MapperSerivce
/// this service stores the Custom errror Type in a field using ```std::marker::PhantomData``` 
pub struct MapperService<E> {
    _error: PhantomData<E>,
}

impl<E> MapperService<E> {
    pub fn new() -> MapperService<E> {
        MapperService {
            _error: PhantomData,
        }
    }
}

impl<S, B, E> Transform<S, ServiceRequest> for MapperService<E>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody,
    E: ResponseError + From<Error> + 'static,
{
    type Response = <MapperMiddleware<S, E> as Service<ServiceRequest>>::Response;
    type Error = Error;
    type Transform = MapperMiddleware<S, E>;
    type InitError = ();
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(MapperMiddleware {
            service: Rc::new(service),
            _error: self._error,
        })
    }
}
