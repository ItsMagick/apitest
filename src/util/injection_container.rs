use once_cell::sync::Lazy;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;
use std::fmt::Debug;
use std::ops::Deref;
use std::marker::PhantomData;
use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;

/**
    * I tried to implement a simple dependency injection container for the project.
    * But I had to reimplement it to support the parameterized and qualified services for more comfortable dependency injection.
    * And I gave up on the idea of using it in the project.
*/


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Qualifier(pub &'static str);

pub struct InjectionContainer {
    services: Mutex<HashMap<(TypeId, Qualifier), Box<dyn Any + Send + Sync>>>,
    factories: Mutex<HashMap<(TypeId, Qualifier), Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any + Send + Sync> + Send + Sync>>>,
}

/**
    * The InjectionContainer is a simple implementation of a dependency injection container.
    * It is a singleton that holds services and factories.
    * The services are stored in a HashMap with the TypeId of the service and an optional qualifier as the key.
    * Had to be Reimplemented to support the parameterized and qualified services for more comfortable dependency injection.
 */
impl InjectionContainer {
    fn new() -> Self {
        Self {
            services: Mutex::new(HashMap::new()),
            factories: Mutex::new(HashMap::new()),
        }
    }

    fn register<T: Any + Send + Sync>(&self, instance: T, qualifier: Qualifier) {
        let mut services = self.services.lock().unwrap();
        services.insert((TypeId::of::<T>(), qualifier), Box::new(instance));
    }

    fn register_factory<T, P, F>(&self, factory: F, qualifier: Qualifier)
    where
        T: Any + Send + Sync,
        P: Any + Send + Sync,
        F: Fn(Box<dyn Any>) -> T + Send + Sync + 'static,
    {
        let mut factories = self.factories.lock().unwrap();
        factories.insert((TypeId::of::<T>(), qualifier), Box::new(move |param| Box::new(factory(param))));
    }

    fn get<T: Any + Send + Sync>(&self, qualifier: Qualifier) -> Option<&T> {
        let services = self.services.lock().unwrap();
        services.get(&(TypeId::of::<T>(), qualifier))
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    fn get_with_params<T, P>(&self, param: P, qualifier: Qualifier) -> Option<T>
    where
    T: Any + Send + Sync,
    P: Any + Send + Sync,
    {
        let factories = self.factories.lock().unwrap();
        if let Some(factory) = factories.get(&(TypeId::of::<T>(), qualifier)) {
            let instance = factory(Box::new(param));
            return instance.downcast::<T>().map(|boxed| *boxed).ok();
        }
        None
    }

    fn get_with_qualifier<T: Any + Send + Sync>(&self, qualifier: Qualifier) -> Option<&T> {
        let services = self.services.lock().unwrap();
        services.get(&(TypeId::of::<T>(), qualifier))
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}

pub static KOIN: Lazy<InjectionContainer> = Lazy::new(InjectionContainer::new);

pub trait InjectionComponent{
    fn get<T: Any + Send + Sync>(&self, qualifier: Qualifier) -> T{
        KOIN.get(qualifier).expect("Service not found").clone()
    }
    fn get_with_params<T, P>(&self, param: P, qualifier: Qualifier) -> T
    where
    T: Any + Send + Sync,
    P: Any + Send + Sync,
    {
        KOIN.get_with_params(param, qualifier).expect("Factory not found")
    }
    fn get_with_qualifier<T: Any + Send + Sync>(&self, qualifier: Qualifier) -> T{
        KOIN.get::<T>(qualifier).expect("Service not found").clone()
    }
}


pub struct Inject<T: Any + Send + Sync> {
    qualifier: Qualifier,
    _marker: PhantomData<T>,
}

impl<T: Any + Send + Sync> Inject<T> {
    pub fn new(qualifier: Qualifier) -> Self {
        Inject { qualifier, _marker: PhantomData }
    }
}

impl<T: Any + Send + Sync + Clone> Deref for Inject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if let Some(q) = &self.qualifier {
            KOIN.get_with_qualifier::<T>(!q).expect("Qualified dependency not found")
        } else {
            KOIN.get::<T>(None).expect("Dependency not found")
        }
    }
}

#[rocket::async_trait]
impl<'r, T: Any + Send + Sync + Clone> FromRequest<'r> for Inject<T> {
    type Error = ();

    async fn from_request(req: &'r rocket::request::Request<'_>) -> Outcome<Self, Self::Error> {
        let qualifier = req
            .headers()
            .get_one("X-Qualifier") // Custom header to specify qualifier
            .map(|q| Qualifier(q));

        if KOIN.get::<T>(qualifier.clone()).is_some() {
            Outcome::Success(Inject::new(None))
        } else {
            Outcome::Error((Status::InternalServerError, ()))

        }
    }
}