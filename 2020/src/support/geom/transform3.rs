use crate::support::Point3;

#[derive(Debug, Clone)]
pub struct Transform3
{
    coords: [[i64; 4]; 4],
}

impl Transform3
{
    pub fn identity() -> Transform3
    {
        Transform3
        {
            coords: [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
        }
    }

    pub fn rotations_and_reflections_24() -> impl Iterator<Item = &'static Transform3>
    {
        // https://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
        // via https://www.reddit.com/r/adventofcode/comments/rjpf7f/comment/hqfeyxv/?utm_source=share&utm_medium=web2x&context=3

        const TRANSFORMS: [Transform3; 24] = [

            Transform3 { coords: [[ 1,  0,  0, 0], [ 0,  1,  0, 0], [ 0,  0,  1, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 1,  0,  0, 0], [ 0,  0, -1, 0], [ 0,  1,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 1,  0,  0, 0], [ 0, -1,  0, 0], [ 0,  0, -1, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 1,  0,  0, 0], [ 0,  0,  1, 0], [ 0, -1,  0, 0], [0, 0, 0, 1]] },

            Transform3 { coords: [[ 0, -1,  0, 0], [ 1,  0,  0, 0], [ 0,  0,  1, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0,  0,  1, 0], [ 1,  0,  0, 0], [ 0,  1,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0,  1,  0, 0], [ 1,  0,  0, 0], [ 0,  0, -1, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0,  0, -1, 0], [ 1,  0,  0, 0], [ 0, -1,  0, 0], [0, 0, 0, 1]] },

            Transform3 { coords: [[-1,  0,  0, 0], [ 0, -1,  0, 0], [ 0,  0,  1, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[-1,  0,  0, 0], [ 0,  0, -1, 0], [ 0, -1,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[-1,  0,  0, 0], [ 0,  1,  0, 0], [ 0,  0, -1, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[-1,  0,  0, 0], [ 0,  0,  1, 0], [ 0,  1,  0, 0], [0, 0, 0, 1]] },

            Transform3 { coords: [[ 0,  1,  0, 0], [-1,  0,  0, 0], [ 0,  0,  1, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0,  0,  1, 0], [-1,  0,  0, 0], [ 0, -1,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0, -1,  0, 0], [-1,  0,  0, 0], [ 0,  0, -1, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0,  0, -1, 0], [-1,  0,  0, 0], [ 0,  1,  0, 0], [0, 0, 0, 1]] },

            Transform3 { coords: [[ 0,  0, -1, 0], [ 0,  1,  0, 0], [ 1,  0,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0,  1,  0, 0], [ 0,  0,  1, 0], [ 1,  0,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0,  0,  1, 0], [ 0, -1,  0, 0], [ 1,  0,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0, -1,  0, 0], [ 0,  0, -1, 0], [ 1,  0,  0, 0], [0, 0, 0, 1]] },

            Transform3 { coords: [[ 0,  0, -1, 0], [ 0, -1,  0, 0], [-1,  0,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0, -1,  0, 0], [ 0,  0,  1, 0], [-1,  0,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0,  0,  1, 0], [ 0,  1,  0, 0], [-1,  0,  0, 0], [0, 0, 0, 1]] },
            Transform3 { coords: [[ 0,  1,  0, 0], [ 0,  0, -1, 0], [-1,  0,  0, 0], [0, 0, 0, 1]] },
        ];

        TRANSFORMS.iter()
    }

    pub fn translation(offset: Point3) -> Transform3
    {
        Transform3
        {
            coords: [[1, 0, 0, offset.x], [0, 1, 0, offset.y], [0, 0, 1, offset.z], [0, 0, 0, 0]]
        }
    }

    pub fn append(&self, other: &Transform3) -> Transform3
    {
        let mut result = Transform3::identity();

        for a in 0..4
        {
            for b in 0..4
            {
                result.coords[a][b] =
                    other.coords[a][0] * self.coords[0][b]
                    + other.coords[a][1] * self.coords[1][b]
                    + other.coords[a][2] * self.coords[2][b]
                    + other.coords[a][3] * self.coords[3][b];
            }
        }

        result
    }

    pub fn transform_point(&self, point: Point3) -> Point3
    {
        let x = self.coords[0][0] * point.x
              + self.coords[0][1] * point.y
              + self.coords[0][2] * point.z
              + self.coords[0][3];

        let y = self.coords[1][0] * point.x
              + self.coords[1][1] * point.y
              + self.coords[1][2] * point.z
              + self.coords[1][3];

        let z = self.coords[2][0] * point.x
              + self.coords[2][1] * point.y
              + self.coords[2][2] * point.z
              + self.coords[2][3];

        Point3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::support::Point3;
    use std::collections::HashSet;

    #[test]
    fn test_transform_identity()
    {
        let identity = Transform3::identity();

        for x in [-2, 2]
        {
            for y in [-3, 3]
            {
                for z in [-7, 7]
                {
                    let orig = Point3::new(x, y, z);
                    let trans = identity.transform_point(orig);

                    assert_eq!(trans, orig);
                }
            }
        }
    }

    #[test]
    fn test_transform_translation()
    {
        let translation = Transform3::translation(Point3::new(19, 139, 251));

        for x in [-2, 2]
        {
            for y in [-3, 3]
            {
                for z in [-7, 7]
                {
                    let calc = translation.transform_point(Point3::new(x, y, z));
                    let expected = Point3::new(x + 19, y + 139, z + 251);

                    assert_eq!(calc, expected);
                }
            }
        }
    }

    #[test]
    fn test_transform_rotations_and_reflections()
    {
        // Create the points manually
        
        let mut manual = HashSet::new();

        for x in [-2, 2]
        {
            for y in [-3, 3]
            {
                for z in [-7, 7]
                {
                    let num_reflections =
                        if x < 0 { 1 } else { 0 }
                        + if y < 0 { 1 } else { 0 }
                        + if z < 0 { 1 } else { 0 };

                    if (num_reflections % 2) == 0
                    {
                        // Even # reflections - choose
                        // axis rotated

                        manual.insert(Point3::new(x, y, z));
                        manual.insert(Point3::new(z, x, y));
                        manual.insert(Point3::new(y, z, x));
                    }
                    else
                    {
                        // Odd # reflections - choose
                        // axis shuffled

                        manual.insert(Point3::new(x, z, y));
                        manual.insert(Point3::new(y, x, z));
                        manual.insert(Point3::new(z, y, x));
                    }
                }
            }
        }

        // Create the points using the transforms

        let mut generated = HashSet::new();
        let point = Point3::new(2, 3, 7);

        for t in Transform3::rotations_and_reflections_24()
        {
            generated.insert(t.transform_point(point));
        }

        // Check they are the same

        assert_eq!(generated.len(), manual.len());
        assert_eq!(generated.len(), 24);

        for p in manual
        {
            assert!(generated.contains(&p))
        }
    }

    #[test]
    fn test_transform_append()
    {
        let t_rot = Transform3::rotations_and_reflections_24().skip(5).next().unwrap().clone();
        let t_trans = Transform3::translation(Point3::new(41, 109, 271));

        let t_rot_trans = t_rot.append(&t_trans);
        let t_trans_rot = t_trans.append(&t_rot);

        for x in [-2, 2]
        {
            for y in [-3, 3]
            {
                for z in [-7, 7]
                {
                    let p = Point3::new(x, y, z);

                    assert_eq!(
                        t_rot.transform_point(p),
                        Point3::new(z, x, y));

                    assert_eq!(
                        t_trans.transform_point(p),
                        Point3::new(x + 41, y + 109, z + 271));

                    assert_eq!(
                        t_rot_trans.transform_point(p),
                        Point3::new(z + 41, x + 109, y + 271));

                    assert_eq!(
                        t_trans_rot.transform_point(p),
                        Point3::new(z + 271, x + 41, y + 109));
                }
            }
        }
    }
}
