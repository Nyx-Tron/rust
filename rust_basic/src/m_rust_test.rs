use std::f64::consts::PI;

pub fn rust_test()
{

}

pub trait  Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        return Rectangle {
            width: width,
            height: height,
        }
    }
}
impl Shape for Rectangle {
    fn name(&self) -> String {
        return "Rectangle".to_string();
    }
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

pub struct Circle {
    radius: f64,
}
impl Circle {
    fn new(radius: f64) -> Circle {
        return Circle {
            radius: radius,
        }
    }
}
impl Shape for Circle {
    fn name(&self) -> String {
        return "Circle".to_string();
    }
    fn area(&self) -> f64 {
        return PI * self.radius * self.radius;
    }
}


pub fn calculate_all_shapes_totally_area<T: Shape>(shapes: &[T]) -> f64
{
    shapes.iter().map(Shape::area).sum()
}
// pub fn calculate_all_shapes_totally_area(shapes: &[&dyn Shape]) -> f64
// {
//     shapes.iter().map(|shape| shape.area()).sum()
// }

#[cfg(test)] // cfg（config) 配置属性
mod tests {
    /*
        m_rust_test 的子模块
    */
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10.0, 20.0);
        assert_eq!(rect.area(), 200.0); // assert_eq! 断言值是否相等
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle::new(10.0);
        assert_eq!(circle.area(), PI * 100.0);
    }

    #[test]
    fn test_calculate_all_shape_totally_area() {
        let rect = Rectangle::new(10.0, 20.0);
        let circle = Circle::new(10.0);
        // let shapes = vec![rect, circle];
        let shapes = vec![rect];
        /* !!!重点：
            <T: Shape> 泛型在接受第一参数后， 就已经单态化（monotonicity），不再支持 Circle
            可以使用 trait 对象解决：
                pub fn calculate_all_shapes_totally_area(shapes: &[&dyn Shape]) -> f64
                {
                    shapes.iter().map(|shape| shape.area()).sum()
                }

                let shapes: Vec<&dyn Shape> = vec![&rect, &circle];
        */

        assert_eq!(calculate_all_shapes_totally_area(&shapes), 200.0);
    }
}