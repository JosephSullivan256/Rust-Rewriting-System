use super::parser::ASTNode;

use std::collections::HashMap;

pub struct Property {
    left: ASTNode,
    right: ASTNode,
    
}

impl Property {
    pub fn new(equivalence: ASTNode) -> Result<Property,&'static str>{
        if let ASTNode::Function(func,nodes) = equivalence {
            if func == "=" && nodes.len() == 2 {
                return Ok(Property{left: nodes[0].clone(), right: nodes[1].clone()});
            } else {
                return Err("Node provided is not a 1 to 1 equivalence.");
            }
        }
        return Err("Node provided is not a function.");
    }

    pub fn inverted(&self) -> Property {
        Property{left: self.right.clone(), right: self.left.clone()}
    }

    pub fn apply_thorough(&self, expression: &ASTNode) -> Vec<ASTNode> {
        let mut possibilities = Vec::new();

        // push root node if "apply" successful
        if let Ok(node) = self.apply(expression) { possibilities.push(node); }

        // replace child node in copy of root for each successful "apply" per child
        if let ASTNode::Function(f,nodes) = expression {
            for (i,node) in nodes.iter().enumerate() {
                for sub_poss in self.apply_thorough(node) {
                    let mut nodes_copy = nodes.clone();
                    nodes_copy[i] = sub_poss;
                    possibilities.push(ASTNode::Function(f.clone(),nodes_copy));
                }
                //mem::replace(&mut node, self.apply_throughout(&node));
                //possibilities.push();
            }
        }
        possibilities
    }

    pub fn apply(&self, expression: &ASTNode) -> Result<ASTNode,()>{
        let mut definitions = Property::deduce_definitions(&self.left, expression)?;
        Ok(Property::replace_symbols(& mut definitions, &self.right))
    }

    pub fn deduce_definitions (base: &ASTNode, expression: &ASTNode) -> Result<HashMap<String,ASTNode>,()> {
        Property::deduce_definitions_recursive(HashMap::new(),base,expression)
    }

    fn deduce_definitions_recursive(mut definitions: HashMap<String, ASTNode>, base: &ASTNode, expression: &ASTNode) -> Result<HashMap<String,ASTNode>,()> {
        match base {
            // if we get down to a symbol in the base via recursion without conflict
            // check if there is conflict with pre-existing definitions put in the vec
            // if not, put symbol in as a definition and return successful
            ASTNode::Symbol(s) => {
                if let Some(def) = definitions.get(s) {
                    if def!=expression { return Err(()); }
                }
                definitions.insert(s.clone(), expression.clone());
            },
            // unlike symbol, a constant in the property does not allow flexibility, so
            // this branch only checks for exact agreement
            ASTNode::Constant(c) => {
                if let ASTNode::Constant(c2) = expression {
                    if c != c2 { return Err(()); }
                } else {
                    return Err(());
                }
            },
            // when base is function, check if expression is a function of the same type
            // and check children recusrively
            ASTNode::Function(f,nodes) => {
                if let ASTNode::Function(f2,nodes2) = expression {
                    if f!=f2 { return Err(()); }
                    for (new_base,new_exp) in nodes.iter().zip(nodes2.iter()) {
                        match Property::deduce_definitions_recursive(definitions,new_base,new_exp) {
                            Ok(defs) => definitions = defs,
                            Err(e) => return Err(e),
                        };
                    }
                } else {
                    return Err(());
                }
            },
        }
        return Ok(definitions);
    }

    // if right hand sign of equivalence introduces new symbol, it will not be replaced
    fn replace_symbols(definitions: &mut HashMap<String,ASTNode>, base: &ASTNode) -> ASTNode{
        match base {
            ASTNode::Symbol(s) => {
                if let Some(def) = definitions.get(s){
                    def.clone()
                } else {
                    definitions.insert(s.clone(),base.clone());
                    base.clone() //simply putting the base is probably a bad idea is probably not a good idea
                }
            },
            ASTNode::Function(f,nodes) => {
                ASTNode::Function(f.clone(),nodes.iter().map(|x| Property::replace_symbols(definitions,x)).collect())
            },
            _ => base.clone()
        }
    }
}