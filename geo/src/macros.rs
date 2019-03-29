macro_rules! point {
    (x: $x:expr, y: $y:expr) => {
        ()
    }; // (y: $y:expr, x: $x:expr) => { () };
}

/// Creates a [`LineString`] containing the given coordinates.
///
/// [`LineString`]: ./struct.LineString.html
///
/// # Examples
///
/// ```
/// use geo::line_string;
///
/// let ls = line_string![
///     (x: -21.95156, y: 64.1446),
///     (x: -21.951, y: 64.14479),
///     (x: -21.95044, y: 64.14527),
///     (x: -21.951445, y: 64.145508)
/// ];
///
/// assert_eq!(
///     ls[1],
///     geo::Coordinate { x: -21.951, y: 64.14479 },
/// );
/// ```
///
/// ```
/// use geo::line_string;
///
/// let coord1 = geo::Coordinate { x: -21.95156, y: 64.1446 };
/// let coord2 = geo::Coordinate { x: -21.951, y: 64.14479 };
/// let coord3 = geo::Coordinate { x: -21.95044, y: 64.14527 };
/// let coord4 = geo::Coordinate { x: -21.951445, y: 64.145508 };
///
/// let ls = line_string![coord1, coord2, coord3, coord4];
///
/// assert_eq!(
///     ls[1],
///     geo::Coordinate { x: -21.951, y: 64.14479 },
/// );
/// ```
#[macro_export]
macro_rules! line_string {
    // TODO: doesn't handle y, x or lng, lat
    // TODO: doesn't handle trailing comma
    // TODO: add to geo-types?
    ($((x: $x:expr, y: $y:expr)),*) => {
        {
            #[allow(unused_mut)]
            let mut v = Vec::<geo::Coordinate<_>>::new();
            $(
                v.push(geo::Coordinate { x: $x, y: $y });
            )*
            geo::LineString(v)
        }
    };
    ($($coord:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut v = Vec::<geo::Coordinate<_>>::new();
            $(
                v.push($coord);
            )*
            geo::LineString(v)
        }
    };
}
