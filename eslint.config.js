// eslint.config.js (Flat Config)
import prettier from 'eslint-config-prettier';
import js from '@eslint/js';
import globals from 'globals';
import ts from 'typescript-eslint';
import pluginVue from 'eslint-plugin-vue';
import vueParser from 'vue-eslint-parser';
import pluginImportX from 'eslint-plugin-import-x';

export default ts.config(
  js.configs.recommended,
  ...ts.configs.recommended,
  ...pluginVue.configs['flat/recommended'],
  prettier,
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node
      },
      parserOptions: {
        projectService: true,
        extraFileExtensions: ['.vue']
      }
    },
    rules: {
      'no-console': ['error', { allow: ['warn', 'error'] }],
      '@typescript-eslint/no-namespace': 'off',
      '@typescript-eslint/no-empty-function': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-unused-vars': [
        'warn',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          caughtErrorsIgnorePattern: '^_'
        }
      ],
      '@typescript-eslint/return-await': ['error', 'always'],
      '@typescript-eslint/promise-function-async': 'error',
      '@typescript-eslint/await-thenable': 'error',

      eqeqeq: ['error', 'always'],
      'import-x/no-cycle': 'error',
      'import-x/order': [
        'error',
        {
          alphabetize: {
            order: 'asc',
            orderImportKind: 'asc',
            caseInsensitive: false
          },
          groups: [
            'index',
            'sibling',
            'parent',
            'internal',
            'external',
            'builtin',
            'object',
            'type'
          ],
          pathGroups: [
            { pattern: 'apps/**', group: 'internal' },
            { pattern: 'crates/**', group: 'internal' }
          ],
          pathGroupsExcludedImportTypes: ['builtin', 'external', 'object', 'type']
        }
      ],
      'func-style': 'off',
      'no-return-await': 'off',
      // Vue-specific rules (add more as needed)
      'vue/multi-word-component-names': 'off',
      'vue/no-v-html': 'off'
    },

    settings: {
      'import-x/extensions': ['.ts', '.vue'],
      'import-x/parsers': {
        '@typescript-eslint/parser': ['.ts', '.vue']
      }
    },
    plugins: {
      'import-x': pluginImportX,
      'vue': pluginVue
    }
  },
  {
    files: ['**/*.vue'],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: ts.parser,
        extraFileExtensions: ['.vue']
      }
    }
  },
  {
    ignores: [
      '**/.*',
      '**/.DS_Store',
      '**/node_modules',
      '**/target',
      '**/build',
      '**/static',
      '**/dist',
      '**/package',
      '**/.env',
      '**/.env.*',
      '!**/.env.example',
      '**/pnpm-lock.yaml',
      '**/package-lock.json',
      '**/yarn.lock',
      '.github',
      '.vscode',
      '**/eslint.config.js',
      '**/vite.config.ts.timestamp-*',
      'target/',
      'crates/',
    ]
  }
);
