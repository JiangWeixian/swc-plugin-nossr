# swc-plugin-nossr
> Optimize `<NoSSR />` component during ssr build stage.

## usage

```
pnpm install swc-plugin-nossr
```

add this plugin to swc config file.

```
{
  "jsc": {
    "experimental": {
      "plugins": [[require.resolve("swc-plugin-nossr"), {}]]
    }
  }
}
```

## features

Replace `<NoSSR />` children element with empty. e.g.

```diff
<NoSSR>
-  <div>hello world</div> 
</NoSSR>
```
