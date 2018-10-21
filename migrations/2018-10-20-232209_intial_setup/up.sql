CREATE TABLE widget (
  id  INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  price INTEGER NOT NULL
);



INSERT INTO widget(name, description, price) VALUES("widget a", "This is some kind of random device", 500);
INSERT INTO widget(name, description, price) VALUES("widget b", "Cheap bargin bin widget", 5);
INSERT INTO widget(name, description, price) VALUES("widget c", "What a nice thingamajig", 1000);