# Jtab

A tiny command line tool written in rust to print json data as a formatted table.

```bash
> echo '[{"id": "1", "name": "Rust"}, {"id": "2", "name": "Jtab"}]' | jtab

+----+------+
| id | name |
+----+------+
| 1  | Rust |
+----+------+
| 2  | Jtab |
+----+------+
```
