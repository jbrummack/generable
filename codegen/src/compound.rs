use syn::Data;

struct CompoundTypeField<'a> {
    r#type: &'a syn::Type,
    field_name: &'a syn::Ident,
}
enum CompoundType<'a> {
    Struct {
        fields: Vec<CompoundTypeField<'a>>,
    },
    Enum {
        options: Vec<Vec<CompoundTypeField<'a>>>,
    },
    Union {
        variants: Vec<CompoundTypeField<'a>>,
    },
}
impl<'a> CompoundType<'a> {
    fn new(derive_data: &'a syn::Data) -> Self {
        match derive_data {
            Data::Struct(data) => {
                let fields: Vec<_> = data
                    .fields
                    .iter()
                    .map(|field| {
                        let field_name = field.ident.as_ref().unwrap();
                        let r#type = &field.ty;
                        CompoundTypeField { r#type, field_name }
                    })
                    .collect();
                Self::Struct { fields }
            }
            Data::Enum(data) => {
                let options: Vec<_> = data
                    .variants
                    .iter()
                    .map(|variant| {
                        let fields: Vec<_> = variant
                            .fields
                            .iter()
                            .map(|field| {
                                let field_name = field.ident.as_ref().unwrap();
                                let r#type = &field.ty;
                                CompoundTypeField { r#type, field_name }
                            })
                            .collect();
                        fields
                    })
                    .collect();
                Self::Enum { options }
            }
            Data::Union(data) => {
                let variants: Vec<_> = data
                    .fields
                    .named
                    .iter()
                    .map(|variant| {
                        let field_name = variant.ident.as_ref().unwrap();
                        let r#type = &variant.ty;
                        CompoundTypeField { r#type, field_name }
                    })
                    .collect();
                Self::Union { variants }
            }
            _ => panic!("Generable can only be derived for structs"),
        }
    }
}
