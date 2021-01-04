import re
import sys
import xmlschema


def main():
    print(f'''\
//! DO NOT EDIT! Generated by {sys.argv[0].rsplit('/', 1)[1]}
#[allow(unused_imports)]
use serde::{{Serialize, Deserialize}};
#[allow(unused_imports)]
use validator::Validate;
''', file=file)

    filenames = [
        'schemas/запрос-авторизация.xsd',
        # 'schemas/запрос-детали по справочнику.xsd',
    ]
    for f in filenames:
        parse_xsd(f)


def parse_xsd(filename):
    xs = xmlschema.XMLSchema(filename)
    print(f'// {filename}\n', file=file)
    # elements = []
    for i in xs.iter_globals():
        result = []
        is_validation = False
        for j in i.iterchildren():
            if j.annotation is not None:
                result.append(f'    /// {j.annotation.documentation[0].text}')
            if j.type.max_length is not None:
                result.append(f'    #[validate(length(max = {j.type.max_length}))]')
                is_validation = True
            typ = {
                'string': 'String',
                'unsignedInt': 'u32',
                'unsignedLong': 'u64',
            }[j.type.base_type.local_name]
            assert j.max_occurs == 1
            if j.min_occurs == 0:
                # result.append('    #[serde(flatten)]')  # todo что за фигня?
                result.append('    #[serde(skip_serializing_if = "Option::is_none")]')
                typ = f'Option<{typ}>'
            elif j.min_occurs == 1:
                pass
            else:
                raise ValueError(j.min_occurs)
            result.append(f'    pub {to_snake_case(j.name)}: {typ},')
        body = '\n'.join(result)
        if i.annotation is not None:
            print(f'/// {i.annotation.documentation[0].text}', file=file)
        # todo +prefix
        print(f'''\
#[derive(Debug, {'Validate, ' if is_validation else ''}Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct {i.name} {{
{body}
}}
''', file=file)
        # if is_validation:
        #     elements.append('    #[validate]')
        # elements.append(f'    pub {to_snake_case(i.name)}: {i.name},')

#     body = '\n'.join(elements)
#     print(f'''\
# /// {filename}
# #[derive(Debug, Validate, Serialize)]
# #[serde(rename = "Root")]
# #[serde(rename_all = "PascalCase")]
# //#[serde(transparent)]
# pub struct Root1 {{
#     //#[serde(flatten)]
# {body}
# }}
# ''', file=file)


def to_snake_case(name: str) -> str:
    """

    >>> to_snake_case('InstitutionID')
    'institution_id'
    """
    return re.sub(r'([A-Z]+)', lambda m: '_' + m.group(1).lower(), name).lstrip('_')


if __name__ == '__main__':
    with sys.stdout as file:
        main()
