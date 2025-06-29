/**
 * @see https://prettier.io/docs/en/configuration.html
 * @type {import("prettier").Config}
 */
const config = {
	useTabs: true,
	singleQuote: true,
	trailingComma: 'none',
	printWidth: 100,
	cssDeclarationSorterOrder: 'smacss',
	plugins: ['prettier-plugin-vue', 'prettier-plugin-css-order'],
	overrides: [
		{ files: '*.vue', options: { parser: 'vue' } }
	],
	endOfLine: 'auto'
};

export default config;
