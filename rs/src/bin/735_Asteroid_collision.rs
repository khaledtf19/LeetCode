fn main() {
    let res = asteroid_collision(vec![10, 2, -5]);
    dbg!(res);
}

pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];

    'astro_loop: for &asteroid in asteroids.iter() {
        while asteroid < 0 && res.len() > 0 && res[res.len() - 1] > 0 {
            let last = res[res.len() - 1];
            if asteroid.abs() >= last {
                res.pop();
            }
            if asteroid.abs() <= last {
                continue 'astro_loop;
            }
        }
        res.push(asteroid)
    }
    res
}
