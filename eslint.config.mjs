import globals from "globals";
import pluginReactConfig from "eslint-plugin-react/configs/recommended.js";
import { fixupConfigRules } from "@eslint/compat";


export default [
  {languageOptions: { globals: {...globals.browser, ...globals.node} }},
  ...fixupConfigRules(pluginReactConfig),
];