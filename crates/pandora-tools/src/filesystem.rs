use std::fs;

pub fn read_file(
    path: &str
)
    -> Result<String, String>
{

    match fs::read_to_string(
        path
    ) {

        Ok(content) => {

            Ok(content)
        }

        Err(error) => {

            Err(
                error.to_string()
            )
        }
    }
}
