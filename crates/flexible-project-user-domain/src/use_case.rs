//! Use cases of the user microservice domain layer.

use async_trait::async_trait;
use auto_impl::auto_impl;

/// Defines operations applicable to the user microservice data.
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait Repository {}
