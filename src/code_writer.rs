use std::io::Write;

pub struct CodeWriter<'a> {
    writer: &'a mut (Write + 'a),
    indent: String,
}

impl<'a> CodeWriter<'a> {
    pub fn new(writer: &'a mut Write) -> CodeWriter<'a> {
        CodeWriter {
            writer: writer,
            indent: "".to_string(), // Two space master race
        }
    }

    pub fn indented<F>(&mut self, cb: F) where F : Fn(&mut CodeWriter) {
        cb(&mut CodeWriter {
            writer: self.writer,
            indent: format!("{}  ", self.indent),
        });
    }

    pub fn comment(&mut self, comment: &str) {
        if comment.is_empty() {
            self.write_line("//");
        } else {
            self.write_line(&format!("// {}", comment));
        }
    }

    pub fn write<S : AsRef<str>>(&mut self, line: S) {
        let s: String = [self.indent.as_ref(), line.as_ref()].concat();
        let _ = self.writer.write_all(s.as_bytes());
    }

    pub fn write_line<S : AsRef<str>>(&mut self, line: S) {
        (if line.as_ref().is_empty() {
            self.writer.write_all("\n".as_bytes())
        } else {
            let s: String = [self.indent.as_ref(), line.as_ref(), "\n"].concat();
            self.writer.write_all(s.as_bytes())
        }).unwrap();
    }

    pub fn write_header(&mut self) {
        self.comment("autogenerated by xdrust");
        self.write_line("#[allow(dead_code)]");
        self.write_line("");
    }

    pub fn alias<S : AsRef<str>, F>(&mut self, name: S, cb: F)
        where F : Fn(&mut CodeWriter) {
            self.write(&format!("pub type {} = ", name.as_ref()));
            cb(self);
            self.write_line(";")
    }

    pub fn pub_enum<S : AsRef<str>, F>(&mut self, name: S, cb: F)
        where F : Fn(&mut CodeWriter) {
            self.write_line("#[derive(Serialize, Deserialize, PartialEq, Debug)]");
            self.expr_block(&format!("pub enum {}", name.as_ref()), cb);
            self.write_line("");
        }

    pub fn pub_struct<S : AsRef<str>, F>(&mut self, name: S, cb: F)
        where F : Fn(&mut CodeWriter) {
            self.write_line("#[derive(Serialize, Deserialize, PartialEq, Debug)]");
            self.expr_block(&format!("pub struct {}", name.as_ref()), cb);
            self.write_line("");
    }

    pub fn var_vec(&mut self, type_: &str) {
        self.write(&format!("Vec<{}>", type_));
    }

    pub fn enum_decl(&mut self, name: &str, val: &str) {
        self.write_line(&format!("{} = {},", name, val));
    }

    pub fn field_decl(&mut self, name: &str, field_type: &str) {
        self.write_line(&format!("{}: {},", name, field_type));
    }

    pub fn expr_block<F>(&mut self, prefix: &str, cb: F)
        where F : Fn(&mut CodeWriter) {
            self.block(&format!("{} {{", prefix), "}", cb);
    }

    pub fn block<F>(&mut self, first_line: &str, last_line: &str, cb: F)
        where F : Fn(&mut CodeWriter) {
            self.write_line(first_line);
            self.indented(cb);
            self.write_line(last_line);
    }
}
