import re
import sys
# from pprint import pprint

import xmlschema


def main(file):
    print(f'''\
//! DO NOT EDIT! Generated by {sys.argv[0].rsplit('/', 1)[1]}
#[allow(unused_imports)]
use::serde::{{Serialize, Deserialize}};
#[allow(unused_imports)]
use validator::Validate;
''', file=file)

    filename = 'schemas/запрос-авторизация.xsd'
    xs = xmlschema.XMLSchema(filename)
    print(f'// from {filename}\n', file=file)
    for i in xs.iter_globals():
        result = []
        is_validation = False
        for j in i.iterchildren():
            result.append(f'    /// {j.annotation.documentation[0].text}')
            if j.type.max_length is not None:
                result.append(f'    #[validate(length(max = {j.type.max_length}))]')
                is_validation = True
            name = to_snake_case(j.name)
            if name != j.name:
                result.append(f'    #[serde(rename = "{j.name}", default)]')

            typ = {
                'string': 'String',
                'unsignedLong': 'u64',
            }[j.type.base_type.local_name]
            assert j.max_occurs == 1
            if j.min_occurs == 0:
                typ = f'Option<{typ}>'
            elif j.min_occurs == 1:
                pass
            else:
                raise ValueError(j.min_occurs)
            result.append(f'    pub {name}: {typ},')
        body = '\n'.join(result)
        print(f'''\
/// {i.annotation.documentation[0].text}
#[derive(Debug, {'Validate, ' if is_validation else ''}Serialize)]
pub struct {i.name} {{
{body}
}}
''', file=file)


def to_snake_case(name: str) -> str:
    """

    >>> to_snake_case('InstitutionID')
    'institution_id'
    """
    return re.sub(r'([A-Z]+)', lambda m: '_' + m.group(1).lower(), name).lstrip('_')


if __name__ == '__main__':
    with sys.stdout as file:
        main(file)
