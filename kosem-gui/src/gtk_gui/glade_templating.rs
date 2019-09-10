use gtk::prelude::*;

pub struct GladeXmlExtractor {
    package: sxd_document::Package,
    xpath: sxd_xpath::XPath,
}

impl GladeXmlExtractor {
    pub fn new(xml_source: &str) -> GladeXmlExtractor {
        let package = sxd_document::parser::parse(xml_source).unwrap();
        let xpath = sxd_xpath::Factory::new().build("//*[@id=$id]").unwrap().unwrap();

        GladeXmlExtractor {package, xpath}
    }

    pub fn extract_as_string(&mut self, element_id: &str) -> String {
        let document = self.package.as_document();
        let mut context = sxd_xpath::Context::new();
        context.set_variable("id", element_id);
        let xpath_result = self.xpath.evaluate(&context, document.root()).unwrap();
        let xpath_result = if let sxd_xpath::Value::Nodeset(xpath_result) = xpath_result {
            xpath_result
        } else {
            panic!("XPath did not return nodeset");
        };

        let result_package = sxd_document::Package::new();
        let result_document = result_package.as_document();
        let result_interface = result_document.create_element("interface");
        result_document.root().append_child(result_interface);

        for element in document.root().children().iter().filter_map(|e| e.element()).next().unwrap().children() {
            if let Some(element) = element.element() {
                let name = element.name().local_part();
                if name != "object" && name != "template" {
                    result_interface.append_child(element);
                }
            }
        }

        for element in xpath_result.iter() {
            let element = element.element().expect("Should be XML element");
            let parent = element.parent().and_then(|parent| {
                let parent = parent.element()?;
                if parent.name().local_part() == "child" {
                    Some(parent)
                } else {
                    None
                }
            });
            if let Some(parent) = parent {
                parent.remove_from_parent();
            } else {
                element.remove_from_parent();
            }
            result_interface.append_child(element);
        }

        package_to_string(&result_package)
    }

    pub fn extract<T: IsA<gtk::Object>>(&mut self, element_id: &'static str) -> GladeFactory<T> {
        GladeFactory {
            source: self.extract_as_string(element_id),
            element_id,
            _phantom: Default::default(),
        }
    }

    pub fn dump_rest(self) -> String {
        package_to_string(&self.package)
    }

    pub fn build_rest(self) -> gtk::Builder {
        gtk::Builder::new_from_string(&self.dump_rest())
    }
}

fn package_to_string(package: &sxd_document::Package) -> String {
    let document = package.as_document();
    let mut output = Vec::new();
    sxd_document::writer::format_document(&document, &mut output).unwrap();
    let output = String::from_utf8(output).unwrap();
    output
}

pub struct GladeFactory<T: IsA<gtk::Object>> {
    source: String,
    element_id: &'static str,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: IsA<gtk::Object>> GladeFactory<T> {
    pub fn build(&self) -> GladeTemplatedInstance<T> {
        GladeTemplatedInstance {
            builder: gtk::Builder::new_from_string(&self.source),
            element_id: self.element_id,
            _phantom: Default::default(),
        }
    }
}

pub struct GladeTemplatedInstance<T: IsA<gtk::Object>> {
    builder: gtk::Builder,
    element_id: &'static str,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: IsA<gtk::Object>> GladeTemplatedInstance<T> {
    pub fn modify_child<C: IsA<gtk::Object>>(self, id: &str, dlg: impl FnOnce(C)) -> Self {
        dlg(self.builder.get_object(id).unwrap());
        self
    }
    pub fn build(self) -> T {
        self.builder.get_object(self.element_id).unwrap()
    }
}
