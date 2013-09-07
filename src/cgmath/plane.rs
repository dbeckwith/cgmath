// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use vector::Vec3;

use std::fmt;

/// A 3-dimendional plane formed from the equation: `Ax + Bx + Cx + D = 0`
///
/// # Fields
///
/// - `normal`: the normal of the plane where:
///   - `normal.x`: corresponds to `A` in the plane equation
///   - `normal.y`: corresponds to `B` in the plane equation
///   - `normal.z`: corresponds to `C` in the plane equation
/// - `distance`: the distance value, corresponding to `D` in the plane equation
#[deriving(Clone, Eq)]
pub struct Plane<S> {
    normal: Vec3<S>,
    distance: S,
}

impl<S: Clone + fmt::Float> ToStr for Plane<S> {
    fn to_str(&self) -> ~str {
        format!("{:f}x + {:f}y + {:f}z + {:f} = 0",
                self.normal.x,
                self.normal.y,
                self.normal.z,
                self.distance)
    }
}