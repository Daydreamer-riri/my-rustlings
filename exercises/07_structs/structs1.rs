struct ColorRegularStruct {
    // 添加字段(fields)，使其能够通过测试 `regular_structs`。
    // 这些字段应具有什么类型？ RGB颜色值的最小值和最大值是多少？
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // (可选)你可以选择性地在此处进行试验。
    let _green = ColorRegularStruct {
        red: 0,
        green: 255,
        blue: 0,
    };
    let _green_tuple = ColorTupleStruct(0, 255, 0);
    let _unit_struct = UnitStruct;
    println!(
        "Regular Struct - R: {}, G: {}, B: {}",
        _green.red, _green.green, _green.blue
    );
    println!(
        "{:?}s are fun! R: {}, G: {}, B: {}",
        _green_tuple, _green_tuple.0, _green_tuple.1, _green_tuple.2
    );
    println!("{:?}s are fun!", _unit_struct);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: 实例化(Instantiate)一个普通结构体。
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: 实例化一个元组结构体。
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 实例化一个单元结构体。
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
