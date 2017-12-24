//! `Quaternion`s are a convenient representation for rotations in 3D spaces.

use num_traits::{Zero, One, Float};
use approx::ApproxEq;
use std::ops::*;
use std::iter::Sum;
use ops::*;

macro_rules! impl_mul_by_vec {
    ($Vec3:ident $Vec4:ident) => {
        /// 3D vectors can be rotated by being premultiplied by a quaternion, **assuming the
        /// quaternion is normalized**.
        /// On `Vec4`s, the `w` element is preserved, so you can safely rotate 
        /// points and directions.
        ///
        /// ```
        /// # extern crate vek;
        /// # #[macro_use] extern crate approx;
        /// # use vek::{Quaternion, Vec4};
        /// use std::f32::consts::PI;
        ///
        /// # fn main() {
        /// let v = Vec4::unit_x();
        ///
        /// let q = Quaternion::<f32>::identity();
        /// assert_relative_eq!(q * v, v);
        ///
        /// let q = Quaternion::rotation_z(PI);
        /// assert_relative_eq!(q * v, -v);
        ///
        /// let q = Quaternion::rotation_z(PI * 0.5);
        /// assert_relative_eq!(q * v, Vec4::unit_y());
        ///
        /// let q = Quaternion::rotation_z(PI * 1.5);
        /// assert_relative_eq!(q * v, -Vec4::unit_y());
        ///
        /// let angles = 32;
        /// for i in 0..angles {
        ///     let theta = PI * 2. * (i as f32) / (angles as f32);
        ///
        ///     // See what rotating unit vectors do for most angles between 0 and 2*PI.
        ///     // It's helpful to picture this as a right-handed coordinate system.
        ///
        ///     let v = Vec4::unit_y();
        ///     let q = Quaternion::rotation_x(theta);
        ///     assert_relative_eq!(q * v, Vec4::new(0., theta.cos(), theta.sin(), 0.));
        ///
        ///     let v = Vec4::unit_z();
        ///     let q = Quaternion::rotation_y(theta);
        ///     assert_relative_eq!(q * v, Vec4::new(theta.sin(), 0., theta.cos(), 0.));
        ///
        ///     let v = Vec4::unit_x();
        ///     let q = Quaternion::rotation_z(theta);
        ///     assert_relative_eq!(q * v, Vec4::new(theta.cos(), theta.sin(), 0., 0.));
        /// }
        /// # }
        /// ```
        impl<T: Float + Sum> Mul<$Vec4<T>> for Quaternion<T> {
            type Output = $Vec4<T>;
            fn mul(self, rhs: $Vec4<T>) -> Self::Output {
                let $Vec4 { x, y, z, w } = rhs;
                let $Vec3 { x, y, z }  = self * $Vec3 { x, y, z };
                $Vec4 { x, y, z, w }
            }
        }
        /// 3D vectors can be rotated by being premultiplied by a quaternion, **assuming the
        /// quaternion is normalized**.
        impl<T: Float + Sum> Mul<$Vec3<T>> for Quaternion<T> {
            type Output = $Vec3<T>;
            fn mul(self, rhs: $Vec3<T>) -> Self::Output {
                let $Vec3 { x, y, z } = rhs;
                let v = Self { x, y, z, w: T::zero() };
                let qi = self.conjugate(); // We want the inverse, and assume self is normalized.
                (self * v * qi).into()
            }
        }

    };
}

macro_rules! quaternion_vec3_vec4 {
    ($Vec3:ident $Vec4:ident) => {

        impl_mul_by_vec!{$Vec3 $Vec4}

        /// A `Quaternion` can be created directly from a `Vec4`'s `x`, `y`, `z` and `w` elements.
        /// **You are responsible for ensuring that the resulting `Quaternion` is normalized.**
        impl<T> From<$Vec4<T>> for Quaternion<T> {
            fn from(v: $Vec4<T>) -> Self {
                let $Vec4 { x, y, z, w } = v;
                Self { x, y, z, w }
            }
        }
        /// A `Vec4` can be created directly from a `Quaternion`'s `x`, `y`, `z` and `w` elements.
        impl<T> From<Quaternion<T>> for $Vec4<T> {
            fn from(v: Quaternion<T>) -> Self {
                let Quaternion { x, y, z, w } = v;
                Self { x, y, z, w }
            }
        }
        /// A `Vec3` can be created directly from a `Quaternion`'s `x`, `y` and `z` elements.
        impl<T> From<Quaternion<T>> for $Vec3<T> {
            fn from(v: Quaternion<T>) -> Self {
                let Quaternion { x, y, z, .. } = v;
                Self { x, y, z }
            }
        }
    };
}

macro_rules! quaternion_complete_mod {
    ($mod:ident #[$attrs:meta]) => {

        use vec::$mod::*;

        /// `Quaternion`s are a convenient representation for rotations in 3D spaces.
        ///
        /// **IMPORTANT**: Quaternions are only valid as rotations as long as they are
        /// **normalized** (i.e their magnitude is 1). Most operations assume
        /// this, instead of normalizing inputs behind your back, so be careful.
        ///
        /// They essentially consist of a vector part (`x`, `y`, `z`), and scalar part (`w`).
        /// For unit quaternions, the vector part is the unit axis of rotation scaled by the sine of
        /// the half-angle of the rotation, and the scalar part is the cosine of the half-angle.
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, /*Ord, PartialOrd*/)]
        #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
        #[$attrs]
        pub struct Quaternion<T> { pub x: T, pub y: T, pub z: T, pub w: T }

        /// The default value for a `Quaternion` is the identity.
        ///
        /// ```
        /// # use vek::Quaternion;
        /// assert_eq!(Quaternion::<f32>::identity(), Quaternion::default());
        /// ```
        impl<T: Zero + One> Default for Quaternion<T> {
            fn default() -> Self {
                Self::identity()
            }
        }

        impl<T> Quaternion<T> {
            /// Creates a new `Quaternion` with `x`, `y`, `z` and `w` elements in order.
            ///
            /// **You are responsible for ensuring that the resulting `Quaternion` is normalized.**
            pub fn from_xyzw(x: T, y: T, z: T, w: T) -> Self {
                Self { x, y, z, w }
            }
            /// Creates a new `Quaternion` from a scalar-and-vector pair.
            ///
            /// **You are responsible for ensuring that the resulting `Quaternion` is normalized.**
            pub fn from_scalar_and_vec3<V: Into<Vec3<T>>>(pair: (T, V)) -> Self {
                let Vec3 { x, y, z } = pair.1.into();
                Self { x, y, z, w: pair.0 }
            }
            /// Converts this `Quaternion` into a scalar-and-vector pair by destructuring.
            ///
            /// **Not to be confused with `into_angle_axis()`**.
            pub fn into_scalar_and_vec3(self) -> (T, Vec3<T>) {
                let Self { x, y, z, w } = self;
                (w, Vec3 { x, y, z })
            }
            /// Creates a new `Quaternion` with all elements set to zero.
            ///
            /// Be careful: since it has a magnitude equal to zero, it is not
            /// valid to use for most operations.
            pub fn zero() -> Self where T: Zero {
                Self { 
                    x: T::zero(),
                    y: T::zero(),
                    z: T::zero(),
                    w: T::zero(),
                }
            }
            /// Creates the identity `Quaternion`.
            ///
            /// ```
            /// # extern crate vek;
            /// # #[macro_use] extern crate approx;
            /// # use vek::Quaternion;
            /// use std::f32::consts::PI;
            ///
            /// # fn main() {
            /// let id = Quaternion::<f32>::identity();
            /// assert_eq!(id, Default::default());
            /// assert_relative_eq!(id, id.conjugate());
            /// assert_relative_eq!(id, id.inverse());
            ///
            /// let q = Quaternion::rotation_y(PI);
            /// assert_relative_eq!(id * q, q);
            /// assert_relative_eq!(q * id, q);
            /// # }
            /// ```
            pub fn identity() -> Self where T: Zero + One {
                Self { 
                    x: T::zero(),
                    y: T::zero(),
                    z: T::zero(),
                    w: T::one(),
                }
            }
            /// Gets this `Quaternion`'s conjugate (copy with negated vector part).
            ///
            /// On normalized `Quaternion`s, the conjugate also happens to be the inverse.
            ///
            /// ```
            /// # extern crate vek;
            /// # #[macro_use] extern crate approx;
            /// # use vek::Quaternion;
            /// use std::f32::consts::PI;
            ///
            /// # fn main() {
            /// let p = Quaternion::rotation_x(PI);
            /// let q = Quaternion::rotation_z(PI);
            /// assert_relative_eq!((p*q).conjugate(), q.conjugate() * p.conjugate());
            ///
            /// // Rotation quaternions are normalized, so their conjugate is also their inverse.
            /// assert_relative_eq!(q.conjugate(), q.inverse());
            /// # }
            /// ```
            pub fn conjugate(self) -> Self where T: Neg<Output=T> {
                Self {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w:  self.w,
                }
            }
            /// Gets this `Quaternion`'s inverse, i.e the one that reverses its effect.
            ///
            /// On normalized `Quaternion`s, the inverse happens to be the conjugate.
            ///
            /// ```
            /// # extern crate vek;
            /// # #[macro_use] extern crate approx;
            /// # use vek::Quaternion;
            /// use std::f32::consts::PI;
            ///
            /// # fn main() {
            /// let rot = Quaternion::rotation_y(PI);
            /// let inv = rot.inverse();
            /// assert_relative_eq!(rot*inv, Quaternion::identity());
            /// assert_relative_eq!(inv*rot, Quaternion::identity());
            ///
            /// let p = Quaternion::rotation_x(PI);
            /// let q = Quaternion::rotation_z(PI);
            /// assert_relative_eq!((p*q).inverse(), q.inverse() * p.inverse());
            /// # }
            /// ```
            pub fn inverse(self) -> Self where T: Neg<Output=T> + Copy + Sum + Mul<Output=T> + Div<Output=T> {
                self.conjugate() / self.into_vec4().magnitude_squared()
            }
            /// Gets the dot product between two quaternions.
            pub fn dot(self, q: Self) -> T 
                where T: Copy + Sum + Mul<Output=T>
            {
                self.into_vec4().dot(q.into_vec4())
            }
            /// Gets a normalized copy of this quaternion.
            pub fn normalized(self) -> Self where T: Float + Sum {
                self.into_vec4().normalized().into()
            }

            /// Creates a `Quaternion` that would rotate a `from` direction to `to`.
            ///
            /// ```
            /// # extern crate vek;
            /// # #[macro_use] extern crate approx;
            /// # use vek::{Vec4, Quaternion};
            ///
            /// # fn main() {
            /// let (from, to) = (Vec4::<f32>::unit_x(), Vec4::<f32>::unit_z());
            /// let q = Quaternion::<f32>::from_arc(from, to);
            /// assert_relative_eq!(q * from, to);
            ///
            /// let (from, to) = (Vec4::<f32>::unit_x(), -Vec4::<f32>::unit_x());
            /// let q = Quaternion::<f32>::from_arc(from, to);
            /// assert_relative_eq!(q * from, to);
            /// # }
            /// ```
            pub fn from_arc<V: Into<Vec3<T>>>(from: V, to: V) -> Self 
                where T: Float + Sum
            {
                // From GLM
                let (from, to) = (from.into(), to.into());
                let norm_u_norm_v = (from.dot(from) * to.dot(to)).sqrt();
                let w = norm_u_norm_v + from.dot(to);
                let (Vec3 { x, y, z }, w) = if w < norm_u_norm_v * T::epsilon() {
                    // If we are here, it is a 180° rotation, which we have to handle.
                    if from.x.abs() > from.z.abs() {
                        (Vec3::new(-from.y, from.x, T::zero()), T::zero())
                    } else {
                        (Vec3::new(T::zero(), -from.z, from.y), T::zero())
                    }
                } else {
                    (from.cross(to), w)
                };
                Self { x, y, z, w }.normalized()
            }

            /// Creates a `Quaternion` from an angle and axis.
            pub fn rotation_3d<V: Into<Vec3<T>>>(angle_radians: T, axis: V) -> Self
                where T: Float
            {
                let axis = axis.into();
                let two = T::one() + T::one();
                let Vec3 { x, y, z } = axis * (angle_radians/two).sin();
                let w = (angle_radians/two).cos();
                Self { x, y, z, w }
            }
            pub fn rotation_x(angle_radians: T) -> Self where T: Float + Sum {
                Self::rotation_3d(angle_radians, Vec3::unit_x())
            }
            pub fn rotation_y(angle_radians: T) -> Self where T: Float + Sum {
                Self::rotation_3d(angle_radians, Vec3::unit_y())
            }
            pub fn rotation_z(angle_radians: T) -> Self where T: Float + Sum {
                Self::rotation_3d(angle_radians, Vec3::unit_z())
            }
            pub fn rotated_3d<V: Into<Vec3<T>>>(self, angle_radians: T, axis: V) -> Self where T: Float + Sum {
                Self::rotation_3d(angle_radians, axis) * self
            }
            pub fn rotated_x(self, angle_radians: T) -> Self where T: Float + Sum {
                Self::rotation_x(angle_radians) * self
            }
            pub fn rotated_y(self, angle_radians: T) -> Self where T: Float + Sum {
                Self::rotation_y(angle_radians) * self
            }
            pub fn rotated_z(self, angle_radians: T) -> Self where T: Float + Sum {
                Self::rotation_z(angle_radians) * self
            }
            pub fn rotate_3d<V: Into<Vec3<T>>>(&mut self, angle_radians: T, axis: V) where T: Float + Sum {
                *self = self.rotated_3d(angle_radians, axis);
            }
            pub fn rotate_x(&mut self, angle_radians: T) where T: Float + Sum {
                *self = self.rotated_x(angle_radians);
            }
            pub fn rotate_y(&mut self, angle_radians: T) where T: Float + Sum {
                *self = self.rotated_y(angle_radians);
            }
            pub fn rotate_z(&mut self, angle_radians: T) where T: Float + Sum {
                *self = self.rotated_z(angle_radians);
            }

            /// Convert this quaternion to angle-axis representation,
            /// **assuming the quaternion is normalized.**
            ///
            /// ```
            /// # extern crate vek;
            /// # #[macro_use] extern crate approx;
            /// # use vek::{Quaternion, Vec3};
            /// use std::f32::consts::PI;
            ///
            /// # fn main() {
            /// let q = Quaternion::rotation_x(PI/2.);
            /// let (angle, axis) = q.into_angle_axis();
            /// assert_relative_eq!(angle, PI/2.);
            /// assert_relative_eq!(axis, Vec3::unit_x());
            ///
            /// let q = Quaternion::rotation_3d(PI*4./5., Vec3::new(1., 2., 3.));
            /// let (angle, axis) = q.into_angle_axis();
            /// assert_relative_eq!(angle, PI*4./5.);
            /// assert_relative_eq!(axis, Vec3::new(1., 2., 3.));
            /// # }
            /// ```
            pub fn into_angle_axis(self) -> (T, Vec3<T>) where T: Float {
                // http://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToAngle/
                // Also, Q57 of matrix-quaternion FAQ
                let Self { x, y, z, w } = self;
                let angle = w.acos();
                let angle = angle + angle;
                let s = (T::one() - w*w).sqrt();
                let axis = if s < T::epsilon() {
                    Vec3::unit_x() // Any axis would do
                } else {
                    Vec3 { x, y, z } / s
                };
                (angle, axis)
            }

            pub fn into_vec4(self) -> Vec4<T> {
                self.into()
            }
            pub fn from_vec4(v: Vec4<T>) -> Self {
                v.into()
            }
            pub fn into_vec3(self) -> Vec3<T> {
                self.into()
            }
        }
        
        /// The `Lerp` implementation for `Quaternion` is the "Normalized LERP".
        impl<T, Factor> Lerp<Factor> for Quaternion<T>
            where T: Lerp<Factor,Output=T> + Sum + Float,
                  Factor: Copy
        {
            type Output = Self;
            fn lerp_unclamped_precise(from: Self, to: Self, factor: Factor) -> Self {
                let (from, to) = (from.into_vec4(), to.into_vec4());
                Lerp::lerp_unclamped_precise(from, to, factor).normalized().into()
            }
            fn lerp_unclamped(from: Self, to: Self, factor: Factor) -> Self {
                let (from, to) = (from.into_vec4(), to.into_vec4());
                Lerp::lerp_unclamped(from, to, factor).normalized().into()
            }
        }
        /// The `Lerp` implementation for `Quaternion` is the "Normalized LERP".
        impl<'a, T, Factor> Lerp<Factor> for &'a Quaternion<T>
            // Float implies Copy, so no &'a T here.
            where T: Lerp<Factor,Output=T> + Sum + Float,
                  Factor: Copy
        {
            type Output = Quaternion<T>;
            fn lerp_unclamped_precise(from: Self, to: Self, factor: Factor) -> Quaternion<T> {
                Lerp::lerp_unclamped_precise(*from, *to, factor)
            }
            fn lerp_unclamped(from: Self, to: Self, factor: Factor) -> Quaternion<T> {
                Lerp::lerp_unclamped(*from, *to, factor)
            }
        }
        impl<T> Quaternion<T> 
            where T: Copy + One + Mul<Output=T> + Sub<Output=T> + MulAdd<T,T,Output=T>
        {
            /// Performs linear interpolation **without normalizing the result**,
            /// using an implementation that supposedly yields a more precise result.
            /// 
            /// This is probably not what you're looking for.  
            /// For an implementation that normalizes the result (which is more commonly wanted), see the `Lerp` implementation.
            pub fn lerp_precise_unnormalized(from: Self, to: Self, factor: T) -> Self where T: Clamp + Zero {
                Self::lerp_unclamped_precise_unnormalized(from, to, factor.clamped01())
            }
            /// Performs linear interpolation **without normalizing the result** and without
            /// implicitly constraining `factor` to be between 0 and 1,
            /// using an implementation that supposedly yields a more precise result.
            /// 
            /// This is probably not what you're looking for.  
            /// For an implementation that normalizes the result (which is more commonly wanted), see the `Lerp` implementation.
            pub fn lerp_unclamped_precise_unnormalized(from: Self, to: Self, factor: T) -> Self {
                Vec4::lerp_unclamped_precise(from.into(), to.into(), factor).into()
            }
        }
        impl<T> Quaternion<T> 
            where T: Copy + Sub<Output=T> + MulAdd<T,T,Output=T>
        {
            /// Performs linear interpolation **without normalizing the result**.
            /// 
            /// This is probably not what you're looking for.  
            /// For an implementation that normalizes the result (which is more commonly wanted), see the `Lerp` implementation.
            pub fn lerp_unnormalized(from: Self, to: Self, factor: T) -> Self where T: Clamp + Zero + One {
                Self::lerp_unclamped_unnormalized(from, to, factor.clamped01())
            }
            /// Performs linear interpolation **without normalizing the result** and without
            /// implicitly constraining `factor` to be between 0 and 1.
            /// 
            /// This is probably not what you're looking for.  
            /// For an implementation that normalizes the result (which is more commonly wanted), see the `Lerp` implementation.
            pub fn lerp_unclamped_unnormalized(from: Self, to: Self, factor: T) -> Self {
                Vec4::lerp_unclamped(from.into(), to.into(), factor).into()
            }
        }
        impl<T> Quaternion<T>
            where T: Lerp<T,Output=T> + Sum + Float
        {
            /// Performs spherical linear interpolation without implictly constraining `factor` to
            /// be between 0 and 1.
            ///
            /// ```
            /// # extern crate vek;
            /// # #[macro_use] extern crate approx;
            /// # use vek::Quaternion;
            /// use std::f32::consts::PI;
            ///
            /// # fn main() {
            /// let from = Quaternion::rotation_z(0_f32);
            /// let to = Quaternion::rotation_z(PI*9./10.);
            /// 
            /// let angles = 32;
            /// for i in 0..angles {
            ///     let factor = (i as f32) / (angles as f32);
            ///     let expected = Quaternion::rotation_z(factor * PI*9./10.);
            ///     let slerp = Quaternion::slerp(from, to, factor);
            ///     assert_relative_eq!(slerp, expected);
            /// }
            /// # }
            /// ```
            // From GLM's source code.
            pub fn slerp_unclamped(from: Self, mut to: Self, factor: T) -> Self {
                let mut cos_theta = from.dot(to);
                // If cosTheta < 0, the interpolation will take the long way around the sphere. 
                // To fix this, one quat must be negated.
                if cos_theta < T::zero() {
                    to = -to;
                    cos_theta = -cos_theta;
                }

                // Perform a linear interpolation when cosTheta is close to 1 to avoid side effect of sin(angle) becoming a zero denominator
                if cos_theta > T::one() - T::epsilon() {
                    return Self::lerp_unclamped(from, to, factor);
                }
                let angle = cos_theta.acos();
                (from * ((T::one() - factor) * angle).sin() + to * (factor * angle).sin()) / angle.sin()
            }
            /// Perform spherical linear interpolation, constraining `factor` to
            /// be between 0 and 1.
            pub fn slerp(from: Self, to: Self, factor: T) -> Self where T: Clamp {
                Slerp::slerp(from, to, factor)
            }
        }

        impl<T, Factor> Slerp<Factor> for Quaternion<T> 
            where T: Lerp<T,Output=T> + Sum + Float,
                  Factor: Into<T>
        {
            type Output = Self;
            fn slerp_unclamped(from: Self, to: Self, factor: Factor) -> Self {
                Self::slerp_unclamped(from, to, factor.into())
            }
        }
        impl<'a, T, Factor> Slerp<Factor> for &'a Quaternion<T> 
            where T: Lerp<T,Output=T> + Sum + Float,
                  Factor: Into<T>
        {
            type Output = Quaternion<T>;
            fn slerp_unclamped(from: Self, to: Self, factor: Factor) -> Quaternion<T> {
                Quaternion::slerp_unclamped(*from, *to, factor.into())
            }
        }


        impl<T> Neg for Quaternion<T> where T: Neg<Output=T> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
                }
            }
        }
        impl<T> Div<T> for Quaternion<T> where T: Copy + Div<Output=T> {
            type Output = Self;
            fn div(self, rhs: T) -> Self::Output {
                Self {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                    w: self.w / rhs,
                }
            }
        }


        impl<T> Add for Quaternion<T> where T: Add<Output=T> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                    w: self.w + rhs.w,
                }
            }
        }
        impl<T> Sub for Quaternion<T> where T: Sub<Output=T> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                    w: self.w - rhs.w,
                }
            }
        }

        /// The `Mul` implementation for `Quaternion`s is concatenation, a.k.a Grassman product.
        ///
        /// ```
        /// # extern crate vek;
        /// # #[macro_use] extern crate approx;
        /// # use vek::{Quaternion, Vec4};
        /// use std::f32::consts::PI;
        ///
        /// # fn main() {
        /// let v = Vec4::unit_x();
        /// let p = Quaternion::rotation_y(PI/2.);
        /// let q = Quaternion::rotation_z(PI/2.);
        /// assert_relative_eq!((p*q)*v, p*(q*v));
        /// assert_relative_eq!(p*q*v, Vec4::unit_y());
        /// assert_relative_eq!(q*p*v, -Vec4::unit_z());
        /// # }
        /// ```
        impl<T> Mul for Quaternion<T> 
            where T: Copy + Mul<Output=T> + Sub<Output=T> + Zero + Sum
        {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let ((ps, pv), (qs, qv)) = (
                    self.into_scalar_and_vec3(), 
                    rhs.into_scalar_and_vec3()
                );
                let Vec3 { x, y, z } = qv*ps + pv*qs + pv.cross(qv);
                let w = ps*qs - pv.dot(qv);
                Self { x, y, z, w }
            }
        }

        impl<T> Mul<T> for Quaternion<T>
        where T: Mul<Output=T> + Copy
        {
            type Output = Self;
            fn mul(self, rhs: T) -> Self::Output {
                Self {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                    w: self.w * rhs,
                }
            }
        }

        /*
        // TODO NOTE: Only for orthogonal matrices
        static inline void mat4o_mul_quat(mat4 R, mat4 M, quat q)
        {
            quat_mul_vec3(R[0], q, M[0]);
            quat_mul_vec3(R[1], q, M[1]);
            quat_mul_vec3(R[2], q, M[2]);

            R[3][0] = R[3][1] = R[3][2] = 0.f;
            R[3][3] = 1.f;
        }
        */


        impl<T: ApproxEq> ApproxEq for Quaternion<T> where T::Epsilon: Copy {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> T::Epsilon {
                T::default_epsilon()
            }

            fn default_max_relative() -> T::Epsilon {
                T::default_max_relative()
            }

            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
                   T::relative_eq(&self.w, &other.w, epsilon, max_relative)
                && T::relative_eq(&self.x, &other.x, epsilon, max_relative)
                && T::relative_eq(&self.y, &other.y, epsilon, max_relative)
                && T::relative_eq(&self.z, &other.z, epsilon, max_relative)
            }

            fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                   T::ulps_eq(&self.w, &other.w, epsilon, max_ulps)
                && T::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
                && T::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
                && T::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
            }
        }
    }
}


#[cfg(all(nightly, feature="repr_simd"))]
pub mod repr_simd {
    use super::*;
    use super::super::vec::repr_c::{Vec3 as CVec3, Vec4 as CVec4};
    quaternion_complete_mod!(repr_simd #[repr(simd)]);
    quaternion_vec3_vec4!(Vec3 Vec4);
    quaternion_vec3_vec4!(CVec3 CVec4);
}
pub mod repr_c {
    use super::*;
    quaternion_complete_mod!(repr_c #[repr(C)]);
    quaternion_vec3_vec4!(Vec3 Vec4);

    #[cfg(all(nightly, feature="repr_simd"))]
    use super::super::vec::repr_simd::{Vec3 as SimdVec3, Vec4 as SimdVec4};
    #[cfg(all(nightly, feature="repr_simd"))]
    quaternion_vec3_vec4!(SimdVec3 SimdVec4);
}
pub use self::repr_c::*;
