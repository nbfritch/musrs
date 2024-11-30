import searchIcon from '../svg/search.svg';

export default function GlobalSearch() {
  return (
    <div class="relative w-120 mt-5 mr-5">
        <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
          <img class="h-4 w-4" src={searchIcon} />
        </div>
        <input
          type="search"
          class="block w-full h-7 w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-full bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
          placeholder="Search" />
    </div>
  );
};
