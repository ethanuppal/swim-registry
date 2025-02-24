# Swim library registry

## Usage

Run the backend server:

```rs
cargo run
```

It will serve to port 3000.

## API

You should use `serde_json` with the definitions in [`lib.rs`](./src/lib.rs).

### `POST /signup`

<table>
<tr>
<td> <b>Request</b> </td> <td> <b>Response</b> </td>
</tr>
<tr>
<td>

```json
{
    "username": "<string>",
    "password": "<string>",
}
```

</td>
<td>

**Success**

```json
"Success"
```

**Failure**

```json
{
    "Failure": "<string>"
}
```

</td>
</tr>
</table>

### `POST /login`

<table>
<tr>
<td> <b>Request</b> </td> <td> <b>Response</b> </td>
</tr>
<tr>
<td>

```json
{
    "username": "<string>",
    "password": "<string>",
}
```

</td>
<td>

**Success**

```json
{
    "Success": {
        "token": 123
    }
}
```
(`123` is an example here.)

**Failure**

```json
{
    "Failure": "<string>"
}
```

</td>
</tr>
</table>

### `POST /logout`

<table>
<tr>
<td> <b>Request</b> </td> <td> <b>Response</b> </td>
</tr>
<tr>
<td>

```json
{
    "token": 123
    "all": false
}
```

If `all` is `true`, all current sessions will be logged out.

</td>
<td>

**Success**

```json
"Success"
```

**Failure**

```json
{
    "Failure": "<string>"
}
```

</td>
</tr>
</table>

### `POST /publish`
