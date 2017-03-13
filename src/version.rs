use std::iter::Peekable;
use std::slice::Iter;

use comp_op::CompOp;
use version_part::VersionPart;

/// Version struct. A wrapper for a version number string.
pub struct Version<'a> {
    version: &'a str,
    parts: Vec<VersionPart<'a>>
}

/// Version struct implementation.
impl<'a> Version<'a> {

    /// Create a `Version` instance from a version string.
    ///
    /// The version string should be passed to the `version` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    /// use version_compare::version::Version;
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.compare(&Version::from("1.2.3").unwrap()), CompOp::Eq);
    /// ```
    pub fn from(version: &'a str) -> Option<Self> {
        // Split the version string
        let parts = Self::split_version_str(version);

        // Return nothing if the parts are none
        if parts.is_none() {
            return None;
        }

        // Create and return the object
        Some(Version {
            version: version,
            parts: parts.unwrap()
        })
    }

    /// Split the given version string, in it's version parts.
    /// TODO: Move this method to some sort of helper class, maybe as part of `VersionPart`.
    fn split_version_str(version: &'a str) -> Option<Vec<VersionPart>> {
        // Split the version string, and create a vector to put the parts in
        let split = version.split('.');
        let mut parts = Vec::new();

        // Flag to determine whether this version number contains any number part
        let mut has_number = false;

        // Loop over the parts, and parse them
        for part in split {
            // Skip empty parts
            if part.is_empty() {
                continue;
            }

            // Try to parse the value as an number
            match part.parse::<i32>() {
                Ok(number) => {
                    // Push the number part to the vector, and set the has number flag
                    parts.push(VersionPart::Number(number));
                    has_number = true;
                },
                Err(_) => parts.push(VersionPart::Text(part))
            }
        }

        // The version must contain a number part, if any part was parsed
        if !has_number && !parts.is_empty() {
            return None
        }

        // Return the list of parts
        Some(parts)
    }

    /// Get the original version string.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::version::Version;
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.as_str(), "1.2.3");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.version
    }

    /// Get a specific version part by it's `index`.
    /// An error is returned if the given index is out of bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::version::Version;
    /// use version_compare::version_part::VersionPart;
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.part(0), Ok(&VersionPart::Number(1)));
    /// assert_eq!(ver.part(1), Ok(&VersionPart::Number(2)));
    /// assert_eq!(ver.part(2), Ok(&VersionPart::Number(3)));
    /// ```
    pub fn part(&self, index: usize) -> Result<&VersionPart<'a>, ()> {
        // Make sure the index is in-bound
        if index < 0 || index >= self.parts.len() {
            return Err(());
        }

        // Return the requested part
        Ok(&self.parts[index])
    }

    /// Get a vector of all version parts.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::version::Version;
    /// use version_compare::version_part::VersionPart;
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.parts(), &vec![
    ///     VersionPart::Number(1),
    ///     VersionPart::Number(2),
    ///     VersionPart::Number(3)
    /// ]);
    /// ```
    pub fn parts(&self) -> &Vec<VersionPart<'a>> {
        &self.parts
    }

    /// Get the number of parts in this version string.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::version::Version;
    ///
    /// let ver_a = Version::from("1.2.3").unwrap();
    /// let ver_b = Version::from("1.2.3.4").unwrap();
    ///
    /// assert_eq!(ver_a.part_count(), 3);
    /// assert_eq!(ver_b.part_count(), 4);
    /// ```
    pub fn part_count(&self) -> usize {
        self.parts.len()
    }

    /// Compare this version to the given `other` version.
    ///
    /// This method returns one of the following comparison operators:
    /// - Lt
    /// - Eq
    /// - Gt
    ///
    /// # Examples:
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    /// use version_compare::version::Version;
    ///
    /// assert_eq!(Version::from("1.2").unwrap().compare(&Version::from("1.3.2").unwrap()), CompOp::Lt);
    /// assert_eq!(Version::from("1.9").unwrap().compare(&Version::from("1.9").unwrap()), CompOp::Eq);
    /// assert_eq!(Version::from("0.3.0.0").unwrap().compare(&Version::from("0.3").unwrap()), CompOp::Eq);
    /// assert_eq!(Version::from("2").unwrap().compare(&Version::from("1.7.3").unwrap()), CompOp::Gt);
    /// ```
    pub fn compare(&self, other: &Version) -> CompOp {
        // Compare the versions with their peekable iterators
        Self::compare_iter(
            self.parts.iter().peekable(),
            other.parts.iter().peekable()
        )
    }

    /// Compare this version to the given `other` version,
    /// and check whether the given comparison operator is valid.
    ///
    /// # Examples:
    ///
    /// ```
    /// use version_compare::comp_op::CompOp;
    /// use version_compare::version::Version;
    ///
    /// assert!(Version::from("1.2").unwrap().compare_to(&Version::from("1.3.2").unwrap(), &CompOp::Lt));
    /// assert!(Version::from("1.2").unwrap().compare_to(&Version::from("1.3.2").unwrap(), &CompOp::Le));
    /// assert!(Version::from("1.2").unwrap().compare_to(&Version::from("1.2").unwrap(), &CompOp::Eq));
    /// assert!(Version::from("1.2").unwrap().compare_to(&Version::from("1.2").unwrap(), &CompOp::Le));
    /// ```
    pub fn compare_to(&self, other: &Version, operator: &CompOp) -> bool {
        // Get the comparison result
        let result = self.compare(&other);

        // Match the result against the given operator
        match result {
            CompOp::Eq =>
                match operator {
                    &CompOp::Eq | &CompOp::Le | &CompOp::Ge => true,
                    _ => false
                },
            CompOp::Lt =>
                match operator {
                    &CompOp::Ne | &CompOp::Lt | &CompOp::Le => true,
                    _ => false
                },
            CompOp::Gt =>
                match operator {
                    &CompOp::Ne | &CompOp::Gt | &CompOp::Ge => true,
                    _ => false
                },

            // This should never be reached
            _ => panic!()
        }
    }

    /// Compare two version numbers based on the iterators of their version parts.
    ///
    /// This method returns one of the following comparison operators:
    /// - Lt
    /// - Eq
    /// - Gt
    fn compare_iter(mut iter: Peekable<Iter<VersionPart<'a>>>, mut other_iter: Peekable<Iter<VersionPart<'a>>>) -> CompOp {
        // Iterate through the parts of this version
        let mut other_part: Option<&VersionPart>;

        // Iterate over the iterator, without consuming it
        loop {
            match iter.next() {
                Some(part) => {
                    // Skip this part if it's non-numeric
                    match part {
                        &VersionPart::Number(_) => {},
                        _ => continue
                    }

                    // Get the next numerical part for the other version
                    loop {
                        // Get the next other part
                        other_part = other_iter.next();

                        // Make sure it's a number or none
                        match other_part {
                            Some(val) =>
                                match val {
                                    &VersionPart::Number(_) => break,
                                    _ => {}
                                },
                            None => break
                        }
                    }

                    // If there are no parts left in the other version, try to determine the result
                    if other_part.is_none() {
                        // In the main version: if the current part is zero, continue to the next one
                        match part {
                            &VersionPart::Number(num) =>
                                if num == 0 {
                                    continue;
                                },
                            _ => {}
                        }

                        // The main version is greater
                        return CompOp::Gt;
                    }

                    // Match both part as numbers to destruct their numerical values
                    match part {
                        &VersionPart::Number(num) =>
                            match other_part.unwrap() {
                                &VersionPart::Number(other_num) => {
                                    // Compare the numbers
                                    match num {
                                        n if n < other_num => return CompOp::Lt,
                                        n if n > other_num => return CompOp::Gt,
                                        n if n == other_num => continue,

                                        // This part can't be reached
                                        _ => panic!()
                                    }
                                },

                                // This part can't be reached
                                _ => panic!()
                            },

                        // This part can't be reached
                        _ => panic!()
                    }
                },
                None => break
            }
        }

        // Check whether we should iterate over the other iterator, if it has any items left
        match other_iter.peek() {
            // Compare based on the other iterator
            Some(_) => Self::compare_iter(other_iter, iter).as_flipped(),

            // Nothing more to iterate over, the versions should be equal
            None => CompOp::Eq
        }
    }
}

#[cfg(test)]
mod tests {
    use comp_op::CompOp;
    use test::test_version::{TEST_VERSIONS, TEST_VERSIONS_ERROR};
    use test::test_version_set::TEST_VERSION_SETS;
    use version::Version;

    #[test]
    // TODO: This doesn't really test whether this method fully works
    fn from() {
        // Test whether parsing works for each test version
        for version in TEST_VERSIONS {
            assert!(Version::from(&version.0).is_some());
        }

        // Test whether parsing works for each test invalid version
        for version in TEST_VERSIONS_ERROR {
            assert!(Version::from(&version.0).is_none());
        }
    }

    #[test]
    fn as_str() {
        // Test for each test version
        for version in TEST_VERSIONS {
            // The input version string must be the same as the returned string
            assert_eq!(Version::from(&version.0).unwrap().as_str(), version.0);
        }
    }

    #[test]
    fn part() {
        // Test for each test version
        for version in TEST_VERSIONS {
            // Create a version object
            let ver = Version::from(&version.0).unwrap();

            // Loop through each part
            for i in 0..version.1 {
                assert_eq!(ver.part(i), Ok(&ver.parts[i]));
            }

            // A value outside the range must return an error
            assert!(ver.part(version.1).is_err());
        }
    }

    #[test]
    fn parts() {
        // Test for each test version
        for version in TEST_VERSIONS {
            // The number of parts must match
            assert_eq!(Version::from(&version.0).unwrap().parts().len(), version.1);
        }
    }

    #[test]
    fn part_count() {
        // Test for each test version
        for version in TEST_VERSIONS {
            // The number of parts must match the metadata
            assert_eq!(Version::from(&version.0).unwrap().part_count(), version.1);
        }
    }

    #[test]
    fn compare() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            // Get both versions
            let version_a = Version::from(&entry.0).unwrap();
            let version_b = Version::from(&entry.1).unwrap();

            // Compare them
            assert_eq!(
                version_a.compare(&version_b),
                entry.2.clone()
            );
        }
    }

    #[test]
    fn compare_to() {
        // Compare each version in the version set
        for entry in TEST_VERSION_SETS {
            // Get both versions
            let version_a = Version::from(&entry.0).unwrap();
            let version_b = Version::from(&entry.1).unwrap();

            // Test
            assert!(version_a.compare_to(&version_b, &entry.2));

            // Make sure the inverse operator is not correct
            assert_eq!(version_a.compare_to(&version_b, &entry.2.invert()), false);
        }

        // Assert an exceptional case, compare to not equal
        assert!(
            Version::from("1.2").unwrap().compare_to(
                &Version::from("1.2.3").unwrap(),
            &CompOp::Ne)
        );
    }
}