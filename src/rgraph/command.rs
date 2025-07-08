use uuid::Uuid;
use crate::style::*;

pub enum Identifier {
    Id(Uuid),
    Name(String),
}

fn match_identifier(token: &Token) -> Result<Identifier, String> {
    match token {
        Ident(val) => {},
        QuotedString(val) => {},
        Uuid(val) => {},
        _ => {},
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphOperation {
    Insert(InsertOperation),
    Delete(DeleteOperation),
    Update(UpdateOperation),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InsertOperation {
    Node(Node),
    Edge(Edge),
}

impl InsertOperation {
    fn parse(tokens: &Vec<Token>) -> Result<(), String> {
        assert!(tokens.len() >= 3);
        match tokens[1] {
            Node => {
                
            },
            _ => {},
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeleteOperation {
    Node {
        label: Identifier,
    },
    Edge {
        edge_id: Identifier,
    },
    Cluster {
        label: Identifier,
    },
    GraphAttribute {
        name: String,
    },
    NodeAttribute {
        node_label: String,
        name: String,
    },
    EdgeAttribute {
        edge_id: Uuid,
        name: String,
    },
    ClusterAttribute {
        cluster_label: String,
        name: String,
    },
    NodeFromCluster {
        cluster: String,
        node: String,
    },
    EdgeFromCluster {
        cluster: String,
        edge_id: Uuid,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateOperation {
    GraphAttribute {
        name: String,
        value: String,
    },
    NodeAttribute {
        node_label: String,
        name: String,
        value: String,
    },
    EdgeAttribute {
        edge_id: Uuid,
        name: String,
        value: String,
    },
    ClusterAttribute {
        cluster_label: String,
        name: String,
        value: String,
    },
}

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("insert")]
    Insert,

    #[token("delete")]
    Delete,

    #[token("update")]
    Update,

    #[token("node")]
    Node,

    #[token("edge")]
    Edge,

    #[token("cluster")]
    Cluster,

    #[token("graph_attr")]
    GraphAttr,

    #[token("node_attr")]
    NodeAttr,

    #[token("edge_attr")]
    EdgeAttr,

    #[token("cluster_attr")]
    ClusterAttr,

    #[token("node_to_cluster")]
    NodeToCluster,

    #[token("edge_to_cluster")]
    EdgeToCluster,

    #[token("node_from_cluster")]
    NodeFromCluster,

    #[token("edge_from_cluster")]
    EdgeFromCluster,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_-]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[0-9a-fA-F-]{36}", |lex| Uuid::parse_str(lex.slice()).ok())]
    Uuid(Option<Uuid>),

    #[regex("\"[^\"]*\"", |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    QuotedString(String),

    #[token("=")]
    Eq,

    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().to_string())]
    Number(String),

    #[regex(r"\s+", logos::skip)]
    Whitespace,
}

fn parse_update(tokens: &Vec<Token>) {

}

fn parse_delete(tokens: &Vec<Token>) {

}

impl GraphOperation {
    pub fn parse(input: &str) -> Result<Self, String> {
        use Token::*;
        let tokens: Vec<_> = Token::lexer(input).collect();
    
        if tokens.len() < 3 {
            return Err("Not enough tokens: need at least an operation and a type".into());
        }
    
        match tokens[0] {
            Insert => {
                parse_insert(&tokens)?
            },
    
            Update => {
                parse_update(&tokens)?
            }
            Delete => {
                parse_delete(&tokens)?
            }
    
            op => Err(format!("Unexpected combination: {:?}", op)),
        }
    }
}

pub struct CommandEngine<G: Graph> {
    graph: G,
}

impl<G> CommandEngine<G> {
    pub fn run_command(&mut self, command: &str) -> Result<(), CommandErr> {
        let operation = GraphOperation::parse(command)?;
        match operation {

        }
    }
}