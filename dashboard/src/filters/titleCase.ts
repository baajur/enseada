import Vue from 'vue';

export function titleCase(value: Object): string {
  if (!value) return '';
  const val = value.toString();
  return val.charAt(0).toUpperCase() + val.slice(1);
}

Vue.filter('titleCase', titleCase);

