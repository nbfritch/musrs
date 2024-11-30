export const getUrl = (relativeUrl: string): string => {
  const slashOrEmpty = relativeUrl[0] === '/' ? '' : '/';
  return `${import.meta.env['VITE_API_URL']}${slashOrEmpty}${relativeUrl}`;
};
