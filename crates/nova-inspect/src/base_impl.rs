use std::path::PathBuf;

use crate::Inspectable;

use egui::{DragValue, Response, Ui, Vec2};
use glam::{IVec2, IVec3, IVec4, Quat, UVec2, UVec3, UVec4, Vec3, Vec4};
use legion::Entity;

macro_rules! impl_num {
    ($ty:ty) => {
        impl Inspectable for $ty {
            #[inline]
            fn name(&self) -> &'static str {
                stringify!($ty)
            }

            #[inline]
            fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
                Some(ui.add(DragValue::new(self).speed(0.1)))
            }
        }
    };
}

impl_num!(i8);
impl_num!(u8);
impl_num!(i16);
impl_num!(u16);
impl_num!(i32);
impl_num!(u32);
impl_num!(i64);
impl_num!(u64);
impl_num!(f32);
impl_num!(f64);

impl Inspectable for String {
    #[inline]
    fn name(&self) -> &'static str {
        "String"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        Some(ui.text_edit_singleline(self))
    }
}

impl Inspectable for PathBuf {
    #[inline]
    fn name(&self) -> &'static str {
        "PathBuf"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        let mut string = String::from(self.as_os_str().to_str()?);

        let response = string.inspect(ui)?;

        if response.changed() {
            *self = PathBuf::from(string);
        }

        Some(response)
    }
}

impl Inspectable for bool {
    #[inline]
    fn name(&self) -> &'static str {
        "bool"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        Some(ui.checkbox(self, ""))
    }
}

impl Inspectable for Vec2 {
    #[inline]
    fn name(&self) -> &'static str {
        "Vec2"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        ui.horizontal(|ui| Some(self.x.inspect(ui)? | self.y.inspect(ui)?))
            .inner
    }
}

impl Inspectable for IVec2 {
    #[inline]
    fn name(&self) -> &'static str {
        "IVec2"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        ui.horizontal(|ui| Some(self.x.inspect(ui)? | self.y.inspect(ui)?))
            .inner
    }
}

impl Inspectable for UVec2 {
    #[inline]
    fn name(&self) -> &'static str {
        "UVec2"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        ui.horizontal(|ui| Some(self.x.inspect(ui)? | self.y.inspect(ui)?))
            .inner
    }
}

impl Inspectable for Vec3 {
    #[inline]
    fn name(&self) -> &'static str {
        "Vec3"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        ui.horizontal(|ui| Some(self.x.inspect(ui)? | self.y.inspect(ui)? | self.z.inspect(ui)?))
            .inner
    }
}

impl Inspectable for IVec3 {
    #[inline]
    fn name(&self) -> &'static str {
        "IVec3"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        ui.horizontal(|ui| Some(self.x.inspect(ui)? | self.y.inspect(ui)? | self.z.inspect(ui)?))
            .inner
    }
}

impl Inspectable for UVec3 {
    #[inline]
    fn name(&self) -> &'static str {
        "UVec3"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        ui.horizontal(|ui| Some(self.x.inspect(ui)? | self.y.inspect(ui)? | self.z.inspect(ui)?))
            .inner
    }
}

impl Inspectable for Vec4 {
    #[inline]
    fn name(&self) -> &'static str {
        "Vec4"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        ui.horizontal(|ui| {
            Some(
                self.x.inspect(ui)?
                    | self.y.inspect(ui)?
                    | self.z.inspect(ui)?
                    | self.w.inspect(ui)?,
            )
        })
        .inner
    }
}

impl Inspectable for IVec4 {
    #[inline]
    fn name(&self) -> &'static str {
        "IVec4"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        ui.horizontal(|ui| {
            Some(
                self.x.inspect(ui)?
                    | self.y.inspect(ui)?
                    | self.z.inspect(ui)?
                    | self.w.inspect(ui)?,
            )
        })
        .inner
    }
}

impl Inspectable for UVec4 {
    #[inline]
    fn name(&self) -> &'static str {
        "UVec4"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        ui.horizontal(|ui| {
            Some(
                self.x.inspect(ui)?
                    | self.y.inspect(ui)?
                    | self.z.inspect(ui)?
                    | self.w.inspect(ui)?,
            )
        })
        .inner
    }
}

impl Inspectable for Quat {
    #[inline]
    fn name(&self) -> &'static str {
        "Quat"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        let (x, y, z) = self.to_euler(glam::EulerRot::XYZ);
        let mut vec = Vec3::new(x, y, z);

        let res = vec.inspect(ui);

        *self = Quat::from_euler(glam::EulerRot::XYZ, vec.x, vec.y, vec.z);

        res
    }
}

impl Inspectable for Entity {
    #[inline]
    fn name(&self) -> &'static str {
        "Entity"
    }

    #[inline]
    fn inspect(&mut self, _ui: &mut Ui) -> Option<Response> {
        None
    }
}

impl<T: Inspectable + Default> Inspectable for Option<T> {
    #[inline]
    fn name(&self) -> &'static str {
        "Option"
    }

    #[inline]
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response> {
        if let Some(t) = self {
            ui.horizontal(|ui| {
                let mut response = ui.selectable_label(true, "Some");

                if let Some(r) = t.inspect(ui) {
                    response |= r;
                }

                Some(response)
            })
            .inner
        } else {
            let response = ui.selectable_label(false, "None");

            if response.changed() {
                *self = Some(T::default());
            }

            Some(response)
        }
    }
}
