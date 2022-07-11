use matrix::SMatrix;
use std::fmt::Write;
use std::path::Path;

const HEADER: &str = r#"<svg height="1000" width="1000">
"#;
const END: &str = r#"</svg>
"#;

#[derive(Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub struct Svg<T, const N: usize> {
    lines: Vec<(Point<T>, Point<T>)>,
    labels: Option<[&'static str; N]>,
    vertices: [Point<T>; N],
}

impl<const N: usize> Svg<f64, N> {
    pub fn from_graph(
        graph: SMatrix<f64, N>,
        x: [f64; N],
        y: [f64; N],
        labels: Option<[&'static str; N]>,
    ) -> Self {
        let mut lines = vec![];

        for i in 0..(N - 1) {
            for j in (i + 1)..N {
                if graph.0[i][j] > 0.0 {
                    lines.push((Point::<f64>::new(x[i], y[i]), Point::<f64>::new(x[j], y[j])))
                }
            }
        }

        let mut vertices = [Point::<f64>::new(0., 0.); N];
        for i in 0..N {
            vertices[i] = Point::<f64>::new(x[i], y[i]);
        }

        Self {
            lines,
            labels,
            vertices,
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut svg = String::from(HEADER);

        if let Some(labels) = &self.labels {
            for (vertex, label) in self.vertices.iter().zip(labels) {
                let _ = write!(
                    svg,
                    r#"<text x="{:.0}" y="{:.0}" font-family="monospace" font-size="24">{}</text>"#,
                    (vertex.x + 1.) * 500.,
                    (vertex.y + 1.) * 500.,
                    label,
                );
                svg.push('\n');
            }
        }

        for line in &self.lines {
            let _ = write!(
                svg,
                r#"<line x1="{:.0}" y1="{:.0}" x2="{:.0}" y2="{:.0}" stroke="black" />"#,
                (line.0.x + 1.) * 500.,
                (line.0.y + 1.) * 500.,
                (line.1.x + 1.) * 500.,
                (line.1.y + 1.) * 500.
            );
            svg.push('\n');
        }
        svg.push_str(END);

        std::fs::write(path, svg)?;
        Ok(())
    }
}

#[test]
fn svg() {
    const ROW0: [f64; 5] = [0., 1., 1., 1., 1.];
    const ROW1: [f64; 5] = [1., 0., 1., 1., 1.];
    const ROW2: [f64; 5] = [1., 1., 0., 1., 1.];
    const ROW3: [f64; 5] = [1., 1., 1., 0., 1.];
    const ROW4: [f64; 5] = [1., 1., 1., 1., 0.];
    const LABELS: [&str; 5] = ["0", "1", "2", "3", "4"];
    const MATRIX: [[f64; 5]; 5] = [ROW0, ROW1, ROW2, ROW3, ROW4];

    let graph = SMatrix::from_raw(MATRIX);
    let out = graph.spectral_layout::<2>(1e-8);

    assert!(Svg::from_graph(graph, out[0], out[1], Some(LABELS))
        .save("foo.svg")
        .is_ok());
}
