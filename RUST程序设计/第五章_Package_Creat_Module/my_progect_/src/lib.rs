mod front_of_house;  // 定义外部模块 模块名与文件名相同

pub fn eat_at_restaurant() {

    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

}

// rust 访问上级函数或者模块  使用super关键字哦
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        super::serve_order();  // 使用super::关键字对上级函数或者模块进行访问
        crate::serve_order();  // 通过绝对路径进行访问问
        cook_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,  // 在结构体字段前面添加pub关键字表示此结构体字段公有
        seasonal_fruit: String,  // 此处没添加默认私有
    }

    impl Breakfast {
        // 定义方法里面的关联函数
        pub fn summer(toast: &str) -> Breakfast {  // &str -> 字符串切片类型
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_() {
    let mut meal = back_of_house::Breakfast::summer("rey");

    meal.toast = String::from("wheat");

    println!("{}",meal.toast);
    // meal.seasonal_fruit = String::from("bl");  // 此处声明的结构体的这个字段是私有的
}
