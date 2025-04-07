use {
    migration::Nullable,
    sea_orm::{ActiveValue, Value},
    serde::{
        de::{Error, Visitor},
        Deserialize, Deserializer, Serialize,
    },
    std::{fmt, marker::PhantomData, ops::Deref},
};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum UpdateValue<T> {
    Set(T),
    #[default]
    Unset,
}

impl<T> UpdateValue<T> {
    #[inline]
    pub fn map<U, F>(self, f: F) -> UpdateValue<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Set(x) => UpdateValue::Set(f(x)),
            Self::Unset => UpdateValue::Unset,
        }
    }

    #[inline]
    pub const fn as_ref(&self) -> UpdateValue<&T> {
        match self {
            UpdateValue::Set(x) => UpdateValue::Set(x),
            UpdateValue::Unset => UpdateValue::Unset,
        }
    }

    #[inline]
    pub fn as_deref(&self) -> UpdateValue<&T::Target>
    where
        T: Deref,
    {
        match self.as_ref() {
            UpdateValue::Set(t) => UpdateValue::Set(t.deref()),
            UpdateValue::Unset => UpdateValue::Unset,
        }
    }
}

impl<T: Into<Value>> From<UpdateValue<T>> for ActiveValue<T> {
    fn from(value: UpdateValue<T>) -> Self {
        match value {
            UpdateValue::Set(x) => ActiveValue::Set(x),
            UpdateValue::Unset => ActiveValue::NotSet,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub enum UpdateOption<T> {
    Set(Option<T>),
    #[default]
    Unset,
}

impl<T> UpdateOption<T> {
    #[inline]
    pub fn map<U, F>(self, f: F) -> UpdateOption<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Set(Some(x)) => UpdateOption::Set(Some(f(x))),
            Self::Set(None) => UpdateOption::Set(None),
            Self::Unset => UpdateOption::Unset,
        }
    }

    #[inline]
    pub const fn as_ref(&self) -> UpdateOption<&T> {
        match self {
            UpdateOption::Set(Some(x)) => UpdateOption::Set(Some(x)),
            UpdateOption::Set(None) => UpdateOption::Set(None),
            UpdateOption::Unset => UpdateOption::Unset,
        }
    }

    #[inline]
    pub fn as_deref(&self) -> UpdateOption<&T::Target>
    where
        T: Deref,
    {
        match self.as_ref() {
            UpdateOption::Set(Some(t)) => UpdateOption::Set(Some(t.deref())),
            UpdateOption::Set(None) => UpdateOption::Set(None),
            UpdateOption::Unset => UpdateOption::Unset,
        }
    }
}

impl<T: Into<Value> + Nullable> From<UpdateOption<T>> for ActiveValue<Option<T>> {
    fn from(value: UpdateOption<T>) -> Self {
        match value {
            UpdateOption::Set(x) => ActiveValue::Set(x),
            UpdateOption::Unset => ActiveValue::NotSet,
        }
    }
}

struct UpdateOptionVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for UpdateOptionVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = UpdateOption<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an UpdateValue")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(UpdateOption::Set(None))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(UpdateOption::Set(None))
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|x| UpdateOption::Set(Some(x)))
    }

    fn __private_visit_untagged_option<D>(self, deserializer: D) -> Result<Self::Value, ()>
    where
        D: Deserializer<'de>,
    {
        Ok(UpdateOption::Set(T::deserialize(deserializer).ok()))
    }
}

impl<'de, T> Deserialize<'de> for UpdateOption<T>
where
    T: Deserialize<'de> + fmt::Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(UpdateOptionVisitor::<T> {
            marker: PhantomData,
        })
    }
}
