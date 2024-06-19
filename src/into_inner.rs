pub trait Inner {
    type Result;
    fn inner(&self) -> Self::Result;
}
pub trait InnerMost {
    type Result;
    fn inner_most(&self) -> Self::Result;
}

pub trait IntoInner {
    type IntoInnerResult;
    fn into_inner(self) -> Self::IntoInnerResult;
}

pub trait IntoInnerMost: IntoInner + Sized {
    type IntoInnerMostResult = Self::IntoInnerResult;
    fn into_inner_most(self) -> Self::IntoInnerMostResult;
}
