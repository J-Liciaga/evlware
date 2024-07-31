"use client";

import {
	createContext,
	type ReactNode,
	type ReactElement,
	useContext,
} from "react";

type I18nContextProps = {
	t: string | null;
};

const I18nContext = createContext<I18nContextProps>({
	t: null,
});

type I18nContextProviderProps = {
	children: ReactNode | ReactElement;
	value: string;
};

export default function I18nContextProvider({
	children,
	value,
}: I18nContextProviderProps) {
	return (
		<I18nContext.Provider
			value={{
				t: value,
			}}
		>
			{children}
		</I18nContext.Provider>
	);
}

export const useI18n = () => useContext(I18nContext);
