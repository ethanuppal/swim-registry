# Swim library registry

## Usage

Run the backend server:

```rs
cargo run
```

## API

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

```json
"Success"
```

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

```json
{
    "Success": {
        "token": 123
    }
}
```
(`123` is an example here.)

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

```json
"Success"
```

```json
{
    "Failure": "<string>"
}
```

</td>
</tr>
</table>

### `POST /publish`
