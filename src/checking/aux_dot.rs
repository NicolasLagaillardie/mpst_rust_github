// use dot;
// use std::fs::File;
// use std::io::Write;

// type Nd = usize;
// type Ed<'a> = &'a (usize, usize);
// struct Graph {
//     nodes: Vec<&'static str>,
//     edges: Vec<(usize, usize)>,
// }

// pub fn render_to<W: Write>(output: &mut W) {
//     let nodes = vec!["{x,y}", "{x}", "{y}", "{}"];
//     let edges = vec![(0, 1), (0, 2), (1, 3), (2, 3)];
//     let graph = Graph {
//         nodes: nodes,
//         edges: edges,
//     };

//     dot::render(&graph, output).unwrap()
// }

// impl<'a> dot::Labeller<'a, Nd, Ed<'a>> for Graph {
//     fn graph_id(&'a self) -> dot::Id<'a> {
//         dot::Id::new("example2").unwrap()
//     }
//     fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
//         dot::Id::new(format!("N{}", n)).unwrap()
//     }
//     fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
//         dot::LabelText::LabelStr(self.nodes[*n].into())
//     }
//     fn edge_label<'b>(&'b self, _: &Ed) -> dot::LabelText<'b> {
//         dot::LabelText::LabelStr("&sube;".into())
//     }
// }

// impl<'a> dot::GraphWalk<'a, Nd, Ed<'a>> for Graph {
//     fn nodes(&self) -> dot::Nodes<'a, Nd> {
//         (0..self.nodes.len()).collect()
//     }
//     fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
//         self.edges.iter().collect()
//     }
//     fn source(&self, e: &Ed) -> Nd {
//         e.0
//     }
//     fn target(&self, e: &Ed) -> Nd {
//         e.1
//     }
// }
// pub fn main() {
//     let mut f = File::create("example2.dot").unwrap();
//     render_to(&mut f)
// }
