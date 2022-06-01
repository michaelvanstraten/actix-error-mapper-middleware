use actix_web::{
    body::MessageBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    Error, ResponseError,
};

use futures_util::{future::MapErr, TryFutureExt};
use std::{marker::PhantomData, rc::Rc};

pub struct MapperMiddleware<S, E> {
    pub service: Rc<S>,
    pub _error: PhantomData<E>,
}

impl<S, B, E> Service<ServiceRequest> for MapperMiddleware<S, E>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    E: From<S::Error> + ResponseError + 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = MapErr<S::Future, fn(Error) -> Error>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        service.call(req).map_err(|error| E::from(error).into())
    }
}
