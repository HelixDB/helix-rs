QUERY addUser (name: String, age: I32) =>
    res <- AddN<User>({name:name,age:age})
    RETURN res