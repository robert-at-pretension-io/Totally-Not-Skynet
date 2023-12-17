import svelte from "rollup-plugin-svelte";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import livereload from "rollup-plugin-livereload";
import terser from "@rollup/plugin-terser";
import autoPreprocess from "svelte-preprocess";
import typescript from "@rollup/plugin-typescript";
import serve from "rollup-plugin-serve";
import replace from "@rollup/plugin-replace";


const production = !process.env.ROLLUP_WATCH;

export default {
  input: "src/main.js",
  output: {
    sourcemap: true,
    format: "iife",
    name: "app",
    file: "public/bundle.js",
  },
  plugins: [
    replace({
      // Replace `process.env.ENVIRONMENT` in your Svelte components
      "process.env.ENVIRONMENT": JSON.stringify(process.env.ENVIRONMENT),
      preventAssignment: true,
    }),
    svelte({
      dev: !production,
      preprocess: autoPreprocess(),
      css: (css) => {
        css.write("bundle.css");
      },
    }),
    resolve({
      browser: true,
      dedupe: ["svelte"],
    }),
    commonjs(),
    typescript({ sourceMap: !production }),

    // Serve your app and enable livereload in development
    !production && serve({
      contentBase: ['public'],
      port: 5000,
      open: true,
    }),
    !production && livereload('public'),

    // Minify for production
    production && terser()
  ],
  watch: {
    clearScreen: false,
  },
};
