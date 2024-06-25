use ash::vk;
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct DevicePointer(pub(crate) vk::DeviceAddress);
impl DevicePointer {
    pub const fn dangling() -> Self { DevicePointer(0) }
    #[inline]
    pub fn addr(self) -> usize { self.0 as usize }
    pub fn offset_ptr(self, ptr1: isize, ptr2: isize) -> DevicePointer {
        let offset = ptr1-ptr2;
        DevicePointer((self.0 as isize).checked_add(offset).unwrap() as u64)
    }
    pub fn from_raw(address: u64) -> Self {
        Self(address)
    }
}
impl std::ops::Add for DevicePointer {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl std::ops::Sub for DevicePointer {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl std::ops::Mul for DevicePointer {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl std::ops::Div for DevicePointer {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl std::ops::Shl for DevicePointer {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0 << rhs.0)
    }
}
impl std::ops::Shr for DevicePointer {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        Self(self.0 >> rhs.0)
    }
}

impl std::ops::Add<usize> for DevicePointer {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u64)
    }
}
impl std::ops::Sub<usize> for DevicePointer {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs as u64)
    }
}
impl std::ops::Mul<usize> for DevicePointer {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as u64)
    }
}
impl std::ops::Div<usize> for DevicePointer {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}
impl std::ops::Shl<usize> for DevicePointer {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs as u64)
    }
}
impl std::ops::Shr<usize> for DevicePointer {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs as u64)
    }
}