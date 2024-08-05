mod shapes {
    mod triangle {
        use crate::shapes::triangle::{AbstractTriangle, InvalidTriangleError};
        use crate::utility::MaybeTwo;

        #[derive(Copy, Clone, Debug)]
        pub struct SolvedTriangle<T> {
            a: T,
            b: T,
            c: T,
            alpha: T,
            beta: T,
            gamma: T,
            area: T,
            altitude_a: T,
            altitude_b: T,
            altitude_c: T
        }

        impl<T> SolvedTriangle<T> {
            pub fn rotate_left(self) -> Self {
                SolvedTriangle {
                    a: self.c,
                    b: self.a,
                    c: self.b,
                    alpha: self.gamma,
                    beta: self.alpha,
                    gamma: self.beta,
                    area: self.area,    // Area doesn't change
                    altitude_a: self.altitude_c,
                    altitude_b: self.altitude_a,
                    altitude_c: self.altitude_b,
                }
            }

            pub fn mirror(self, axis: &'static str) -> Self {
                match axis {
                    "A" => SolvedTriangle {
                        a: self.a,
                        b: self.c,
                        c: self.b,
                        alpha: self.alpha,
                        beta: self.gamma,
                        gamma: self.beta,
                        area: self.area,    // Area doesn't change
                        altitude_a: self.altitude_a,
                        altitude_b: self.altitude_c,
                        altitude_c: self.altitude_b,
                    },
                    "B" => SolvedTriangle {
                        a: self.c,
                        b: self.b,
                        c: self.a,
                        alpha: self.gamma,
                        beta: self.beta,
                        gamma: self.alpha,
                        area: self.area,    // Area doesn't change
                        altitude_a: self.altitude_c,
                        altitude_b: self.altitude_b,
                        altitude_c: self.altitude_a,
                    },
                    "C" => SolvedTriangle {
                        a: self.b,
                        b: self.a,
                        c: self.c,
                        alpha: self.beta,
                        beta: self.alpha,
                        gamma: self.gamma,
                        area: self.area,    // Area doesn't change
                        altitude_a: self.altitude_b,
                        altitude_b: self.altitude_a,
                        altitude_c: self.altitude_c,
                    },
                    _ => panic!("Invalid triangle mirror axis: {}", axis)
                }
            }
        }

        fn flt_eq(l: f64, r: f64) -> bool {
            ((l - r).abs() / (l + r).abs()) < (f64::EPSILON * 5.0)  // x5 to make the tests work, errors accumulate but using fused-arithmetic makes the code less flexible w.r.t. generics
        }

        fn assert_abstract_impl<R: AbstractTriangle<f64>>(solution: &SolvedTriangle<f64>, found: R) {
            let len_a_solution = found.length_a();
            assert!(len_a_solution.any_is(|s| flt_eq(s, solution.a)), "Length-a solution {:?} does not contain {}", len_a_solution, solution.a);

            let len_b_solution = found.length_b();
            assert!(len_b_solution.any_is(|s| flt_eq(s, solution.b)), "Length-b solution {:?} does not contain {}", len_b_solution, solution.b);

            let len_c_solution = found.length_c();
            assert!(len_c_solution.any_is(|s| flt_eq(s, solution.c)), "Length-c solution {:?} does not contain {}", len_c_solution, solution.c);

            let angle_alpha_solutions = found.angle_alpha();
            let alpha_degree_expected = solution.alpha.to_degrees();
            let degree_values = angle_alpha_solutions.map(f64::to_degrees);
            assert!(angle_alpha_solutions.any_is(|s| flt_eq(s, solution.alpha)), "Angle-α solution {:?} ({:?}) does not contain {} ({}) ", angle_alpha_solutions, degree_values, solution.alpha, alpha_degree_expected);

            let angle_beta_solutions = found.angle_beta();
            let beta_degree_expected = solution.beta.to_degrees();
            let degree_values = angle_beta_solutions.map(f64::to_degrees);
            assert!(angle_beta_solutions.any_is(|s| flt_eq(s, solution.beta)), "Angle-β solution {:?} ({:?}) does not contain {} ({}) ", angle_beta_solutions, degree_values, solution.beta, beta_degree_expected);

            let angle_gamma_solutions = found.angle_gamma();
            let gamma_degree_expected = solution.gamma.to_degrees();
            let degree_values = angle_gamma_solutions.map(f64::to_degrees);
            assert!(angle_gamma_solutions.any_is(|s| flt_eq(s, solution.gamma)), "Angle-γ solution {:?} ({:?}) does not contain {} ({}) ", angle_gamma_solutions, degree_values, solution.gamma, gamma_degree_expected);

            let area_solutions = found.area();
            assert!(area_solutions.any_is(|s| flt_eq(s, solution.area)), "Area solution {:?} does not contain {}", area_solutions, solution.area);

            let altitude_a_solutions = found.altitude_a();
            assert!(altitude_a_solutions.any_is(|s| flt_eq(s, solution.altitude_a)), "Altitude A solution {:?} does not contain {}", altitude_a_solutions, solution.altitude_a);

            let altitude_b_solutions = found.altitude_b();
            assert!(altitude_b_solutions.any_is(|s| flt_eq(s, solution.altitude_b)), "Altitude B solution {:?} does not contain {}", altitude_b_solutions, solution.altitude_b);

            let altitude_c_solutions = found.altitude_c();
            assert!(altitude_c_solutions.any_is(|s| flt_eq(s, solution.altitude_c)), "Altitude C solution {:?} does not contain {}", altitude_c_solutions, solution.altitude_c);
        }

        fn test_solution(solution: SolvedTriangle<f64>) -> Result<(), InvalidTriangleError> {
            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, b: solution.b, c: solution.c }?);

            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, b: solution.b, alpha: solution.alpha }?);
            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, c: solution.c, alpha: solution.alpha }?);
            assert_abstract_impl(&solution, abstract_triangle!{ b: solution.b, c: solution.c, alpha: solution.alpha }?);
            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, b: solution.b, beta: solution.beta }?);
            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, c: solution.c, beta: solution.beta }?);
            assert_abstract_impl(&solution, abstract_triangle!{ b: solution.b, c: solution.c, beta: solution.beta }?);
            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, b: solution.b, gamma: solution.gamma }?);
            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, c: solution.c, gamma: solution.gamma }?);
            assert_abstract_impl(&solution, abstract_triangle!{ b: solution.b, c: solution.c, gamma: solution.gamma }?);

            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, alpha: solution.alpha, beta: solution.beta }?);
            assert_abstract_impl(&solution, abstract_triangle!{ b: solution.b, alpha: solution.alpha, beta: solution.beta }?);
            assert_abstract_impl(&solution, abstract_triangle!{ c: solution.c, alpha: solution.alpha, beta: solution.beta }?);
            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, alpha: solution.alpha, gamma: solution.gamma }?);
            assert_abstract_impl(&solution, abstract_triangle!{ b: solution.b, alpha: solution.alpha, gamma: solution.gamma }?);
            assert_abstract_impl(&solution, abstract_triangle!{ c: solution.c, alpha: solution.alpha, gamma: solution.gamma }?);
            assert_abstract_impl(&solution, abstract_triangle!{ a: solution.a, beta: solution.beta, gamma: solution.gamma }?);
            assert_abstract_impl(&solution, abstract_triangle!{ b: solution.b, beta: solution.beta, gamma: solution.gamma }?);
            assert_abstract_impl(&solution, abstract_triangle!{ c: solution.c, beta: solution.beta, gamma: solution.gamma }?);

            Ok(())
        }

        // This and the 'isosceles' test are redundant compared to 'scalene', but were written first to figure out an appropriate test structure
        #[test]
        pub fn equilateral() {
            let solution = SolvedTriangle {
                a: 3.0,
                b: 3.0,
                c: 3.0,
                alpha: std::f64::consts::FRAC_PI_3,
                beta: std::f64::consts::FRAC_PI_3,
                gamma: std::f64::consts::FRAC_PI_3,
                area: (3.0f64.sqrt())/4.0 * 3.0 * 3.0,
                altitude_a: 0.5*(3.0f64.sqrt())*3.0,
                altitude_b: 0.5*(3.0f64.sqrt())*3.0,
                altitude_c: 0.5*(3.0f64.sqrt())*3.0,
            };

            // No point in testing the rotated or mirrored version of an equilateral triangle
            test_solution(solution).expect("Test triangle is valid!");
        }

        pub fn test_with_rotation_and_mirror(solution: SolvedTriangle<f64>) {
            test_solution(solution).expect("Test triangle is valid!");
            test_solution(solution.rotate_left()).expect("Test triangle is valid!");
            test_solution(solution.rotate_left().rotate_left()).expect("Test triangle is valid!");

            for axis in ["A", "B", "C"] {
                let mirror = solution.mirror(axis);

                test_solution(mirror).expect("Test triangle is valid!");
                test_solution(mirror.rotate_left()).expect("Test triangle is valid!");
                test_solution(mirror.rotate_left().rotate_left()).expect("Test triangle is valid!");
            }
        }

        #[test]
        pub fn isosceles() {
            let solution = SolvedTriangle {
                a: 5.0,
                b: 5.0,
                c: 6.0,
                alpha: f64::acos(3.0/5.0),
                beta: f64::acos(3.0/5.0),
                gamma: f64::asin(3.0/5.0)*2.0,
                area: 12.0,
                altitude_a: 4.8,
                altitude_b: 4.8,
                altitude_c: 4.0,
            };

            test_with_rotation_and_mirror(solution);
        }

        #[test]
        pub fn scalene() {
            let solution = SolvedTriangle { // See docs/scalene.png
                a: 9.0,
                b: 10.0,
                c: 17.0,
                alpha: f64::acos(8.0/17.0) - f64::acos(8.0/10.0),
                beta: f64::acos(15.0/17.0),
                gamma: std::f64::consts::PI - f64::acos(6.0/10.0),
                area: 36.0,
                altitude_a: 8.0,
                altitude_b: 7.2,
                altitude_c: (2.0*36.0)/ 17.0,
            };

            test_with_rotation_and_mirror(solution);
        }

        #[test]
        pub fn scalene_ambiguity() -> Result<(), InvalidTriangleError> {
            fn assert_eq(left: (f64, Option<f64>), right: (f64, Option<f64>)) {
                let pass = match (left, right) {
                    ((l1, Some(l2)), (r1, Some(r2))) => (flt_eq(l1, r1) && flt_eq(l2, r2)) || (flt_eq(l1, r2) && flt_eq(l2, r1)),
                    ((l1, None), (r1, None)) => flt_eq(l1, r1),
                    _ => false
                };

                assert!(pass, "Ambiguous solution not equal:\n\t{:?} ({:?})\n\t{:?} ({:?})", left, left.map(f64::to_degrees), right, right.map(f64::to_degrees))
            }

            let solution = SolvedTriangle { // See docs/scalene.png
                a: 9.0,
                b: 10.0,
                c: 17.0,
                alpha: f64::acos(8.0/17.0) - f64::acos(8.0/10.0),
                beta: f64::acos(15.0/17.0),
                gamma: std::f64::consts::PI - f64::acos(6.0/10.0),
                area: 36.0,
                altitude_a: 8.0,
                altitude_b: 7.2,
                altitude_c: (2.0*36.0)/ 17.0,
            };

            // Ambiguous values manually confirmed
            let tri = abstract_triangle!{ a: solution.a, b: solution.b, alpha: solution.alpha }?;
            assert_eq(tri.length_c(), (solution.c, Some(1.1176470588235317)));
            assert_eq(tri.angle_beta(), (solution.beta, Some(2.65163532733606)));
            assert_eq(tri.angle_gamma(), (solution.gamma, Some(0.05261943450584479)));

            assert_eq(tri.area(), (solution.area, Some(2.366782006920449)));
            assert_eq(tri.altitude_a(), (solution.altitude_a, Some(0.5259515570934331)));
            assert_eq(tri.altitude_b(), (solution.altitude_b, Some(0.47335640138408974)));
            assert_eq(tri.altitude_c().both(), (solution.altitude_c, None));    // Ambiguity does not change altitude C

            let tri = abstract_triangle!{ a: solution.a, c: solution.c, alpha: solution.alpha }?;
            assert_eq(tri.length_b(), (solution.b, Some(20.8)));
            assert_eq(tri.angle_beta(), (solution.beta, Some(1.7769595438402968)));
            assert_eq(tri.angle_gamma(), (solution.gamma, Some(0.9272952180016127)));

            assert_eq(tri.area(), (solution.area, Some(74.88)));
            assert_eq(tri.altitude_a(), (solution.altitude_a, Some(16.64)));
            assert_eq(tri.altitude_b().both(), (solution.altitude_b, None));   // Ambiguity does not change altitude B
            assert_eq(tri.altitude_c(), (solution.altitude_c, Some(8.809411764705882)));

            let tri = abstract_triangle!{ a: solution.a, b: solution.b, beta: solution.beta }?;
            assert_eq(tri.length_c(), (solution.c, None));
            assert_eq(tri.angle_alpha(), (solution.alpha, None));
            assert_eq(tri.angle_gamma(), (solution.gamma, None));

            assert_eq(tri.area(), (solution.area, None));
            assert_eq(tri.altitude_a(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b(), (solution.altitude_b, None));
            assert_eq(tri.altitude_c().both(), (solution.altitude_c, None));

            let tri = abstract_triangle!{ b: solution.b, c: solution.c, beta: solution.beta }?;
            assert_eq(tri.length_a(), (solution.a, Some(21.0)));
            assert_eq(tri.angle_alpha(), (solution.alpha, Some(1.7243401093344528)));
            assert_eq(tri.angle_gamma(), (solution.gamma, Some(0.9272952180016123)));

            assert_eq(tri.area(), (solution.area, Some(84.0)));
            assert_eq(tri.altitude_a().both(), (solution.altitude_a, None));   // Ambiguity does not change altitude A
            assert_eq(tri.altitude_b(), (solution.altitude_b, Some(16.8)));
            assert_eq(tri.altitude_c(), (solution.altitude_c, Some(9.882352941176471)));

            let tri = abstract_triangle!{ a: solution.a, c: solution.c, gamma: solution.gamma }?;
            assert_eq(tri.length_b(), (solution.b, None));
            assert_eq(tri.angle_alpha(), (solution.alpha, None));
            assert_eq(tri.angle_beta(), (solution.beta, None));

            assert_eq(tri.area(), (solution.area, None));
            assert_eq(tri.altitude_a(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b().both(), (solution.altitude_b, None));
            assert_eq(tri.altitude_c(), (solution.altitude_c, None));

            let tri = abstract_triangle!{ b: solution.b, c: solution.c, gamma: solution.gamma }?;
            assert_eq(tri.length_a(), (solution.a, None));
            assert_eq(tri.angle_alpha(), (solution.alpha, None));
            assert_eq(tri.angle_beta(), (solution.beta, None));

            assert_eq(tri.area(), (solution.area, None));
            assert_eq(tri.altitude_a().both(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b(), (solution.altitude_b, None));
            assert_eq(tri.altitude_c(), (solution.altitude_c, None));

            // Rotated once
            let solution = solution.rotate_left();
            let tri = abstract_triangle!{ a: solution.a, b: solution.b, alpha: solution.alpha }?;
            assert_eq(tri.length_c(), (solution.c, None));
            assert_eq(tri.angle_beta(), (solution.beta, None));
            assert_eq(tri.angle_gamma(), (solution.gamma, None));

            assert_eq(tri.area(), (solution.area, None));
            assert_eq(tri.altitude_a(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b(), (solution.altitude_b, None));
            assert_eq(tri.altitude_c().both(), (solution.altitude_c, None));

            let tri = abstract_triangle!{ a: solution.a, c: solution.c, alpha: solution.alpha }?;
            assert_eq(tri.length_b(), (solution.b, None));
            assert_eq(tri.angle_beta(), (solution.beta, None));
            assert_eq(tri.angle_gamma(), (solution.gamma, None));

            assert_eq(tri.area(), (solution.area, None));
            assert_eq(tri.altitude_a(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b().both(), (solution.altitude_b, None));
            assert_eq(tri.altitude_c(), (solution.altitude_c, None));

            let tri = abstract_triangle!{ a: solution.a, b: solution.b, beta: solution.beta }?;
            assert_eq(tri.length_c(), (solution.c, Some(20.8)));
            assert_eq(tri.angle_alpha(), (solution.alpha, Some(0.9272952180016127)));
            assert_eq(tri.angle_gamma(), (solution.gamma, Some(1.7769595438402968)));

            assert_eq(tri.area(), (solution.area, Some(74.88)));
            assert_eq(tri.altitude_a(), (solution.altitude_a, Some(8.809411764705882)));
            assert_eq(tri.altitude_b(), (solution.altitude_b, Some(16.64)));
            assert_eq(tri.altitude_c().both(), (solution.altitude_c, None));   // Ambiguity does not change altitude C

            let tri = abstract_triangle!{ b: solution.b, c: solution.c, beta: solution.beta }?;
            assert_eq(tri.length_a(), (solution.a, Some(1.1176470588235317)));
            assert_eq(tri.angle_alpha(), (solution.alpha, Some(0.05261943450584479)));
            assert_eq(tri.angle_gamma(), (solution.gamma, Some(2.65163532733606)));

            assert_eq(tri.area(), (solution.area, Some(2.366782006920449)));
            assert_eq(tri.altitude_a().both(), (solution.altitude_a, None));    // Ambiguity does not change altitude A
            assert_eq(tri.altitude_b(), (solution.altitude_b, Some(0.5259515570934331)));
            assert_eq(tri.altitude_c(), (solution.altitude_c, Some(0.47335640138408974)));

            let tri = abstract_triangle!{ a: solution.a, c: solution.c, gamma: solution.gamma }?;
            assert_eq(tri.length_b(), (solution.b, Some(21.0)));
            assert_eq(tri.angle_alpha(), (solution.alpha, Some(0.9272952180016123)));
            assert_eq(tri.angle_beta(), (solution.beta, Some(1.7243401093344528)));

            assert_eq(tri.area(), (solution.area, Some(84.0)));
            assert_eq(tri.altitude_a(), (solution.altitude_a, Some(9.882352941176471)));
            assert_eq(tri.altitude_b().both(), (solution.altitude_b, None));   // Ambiguity does not change altitude B
            assert_eq(tri.altitude_c(), (solution.altitude_c, Some(16.8)));

            let tri = abstract_triangle!{ b: solution.b, c: solution.c, gamma: solution.gamma }?;
            assert_eq(tri.length_a(), (solution.a, None));
            assert_eq(tri.angle_alpha(), (solution.alpha, None));
            assert_eq(tri.angle_beta(), (solution.beta, None));

            assert_eq(tri.area(), (solution.area, None));
            assert_eq(tri.altitude_a().both(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b(), (solution.altitude_b, None));
            assert_eq(tri.altitude_c(), (solution.altitude_c, None));

            // Rotated twice (`solution` already being the rotated version from the previous block)
            let solution = solution.rotate_left();
            let tri = abstract_triangle!{ a: solution.a, b: solution.b, alpha: solution.alpha }?;
            assert_eq(tri.length_c(), (solution.c, Some(21.0)));
            assert_eq(tri.angle_beta(), (solution.beta, Some(0.9272952180016123)));
            assert_eq(tri.angle_gamma(), (solution.gamma, Some(1.7243401093344528)));

            assert_eq(tri.area(), (solution.area, Some(84.0)));
            assert_eq(tri.altitude_a(), (solution.altitude_a, Some(16.8)));
            assert_eq(tri.altitude_b(), (solution.altitude_b, Some(9.882352941176471)));
            assert_eq(tri.altitude_c().both(), (solution.altitude_c, None));   // Ambiguity does not change altitude C

            let tri = abstract_triangle!{ a: solution.a, c: solution.c, alpha: solution.alpha }?;
            assert_eq(tri.length_b(), (solution.b, None));
            assert_eq(tri.angle_beta(), (solution.beta, None));
            assert_eq(tri.angle_gamma(), (solution.gamma, None));

            assert_eq(tri.area(), (solution.area, None));
            assert_eq(tri.altitude_a(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b().both(), (solution.altitude_b, None));
            assert_eq(tri.altitude_c(), (solution.altitude_c, None));

            let tri = abstract_triangle!{ a: solution.a, b: solution.b, beta: solution.beta }?;
            assert_eq(tri.length_c(), (solution.c, None));
            assert_eq(tri.angle_alpha(), (solution.alpha, None));
            assert_eq(tri.angle_gamma(), (solution.gamma, None));

            assert_eq(tri.area(), (solution.area, None));
            assert_eq(tri.altitude_a(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b(), (solution.altitude_b, None));
            assert_eq(tri.altitude_c().both(), (solution.altitude_c, None));

            let tri = abstract_triangle!{ b: solution.b, c: solution.c, beta: solution.beta }?;
            assert_eq(tri.length_a(), (solution.a, None));
            assert_eq(tri.angle_alpha(), (solution.alpha, None));
            assert_eq(tri.angle_gamma(), (solution.gamma, None));

            assert_eq(tri.area(), (solution.area, None));
            assert_eq(tri.altitude_a().both(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b(), (solution.altitude_b, None));
            assert_eq(tri.altitude_c(), (solution.altitude_c, None));

            let tri = abstract_triangle!{ a: solution.a, c: solution.c, gamma: solution.gamma }?;
            assert_eq(tri.length_b(), (solution.b, Some(1.1176470588235317)));
            assert_eq(tri.angle_alpha(), (solution.alpha, Some(2.65163532733606)));
            assert_eq(tri.angle_beta(), (solution.beta, Some(0.05261943450584479)));

            assert_eq(tri.area(), (solution.area, Some(2.366782006920449)));
            assert_eq(tri.altitude_a(), (solution.altitude_a, Some(0.47335640138408974)));
            assert_eq(tri.altitude_b().both(), (solution.altitude_b, None));    // Ambiguity does not change altitude B
            assert_eq(tri.altitude_c(), (solution.altitude_c, Some(0.5259515570934331)));

            let tri = abstract_triangle!{ b: solution.b, c: solution.c, gamma: solution.gamma }?;
            assert_eq(tri.length_a(), (solution.a, Some(20.8)));
            assert_eq(tri.angle_alpha(), (solution.alpha, Some(1.7769595438402968)));
            assert_eq(tri.angle_beta(), (solution.beta, Some(0.9272952180016127)));

            assert_eq(tri.area(), (solution.area, Some(74.88)));
            assert_eq(tri.altitude_a().both(), (solution.altitude_a, None));
            assert_eq(tri.altitude_b(), (solution.altitude_b, Some(8.809411764705882)));
            assert_eq(tri.altitude_c(), (solution.altitude_c, Some(16.64)));

            Ok(())
        }
    }
}
