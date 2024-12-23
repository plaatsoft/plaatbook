/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

// eslint-disable-next-line import/named
import { JSX, Component } from 'preact';

interface InfiniteListProps<T> {
    items: T[];
    fetchPage: (page: number) => Promise<void>;
    template: (item: T, index: number) => JSX.Element;
}

export class InfiniteList<T> extends Component<InfiniteListProps<T>> {
    page = 1;
    loading = false;

    constructor(props: InfiniteListProps<T>) {
        super(props);
        this.checkScroll = this.checkScroll.bind(this);
    }

    async getPage(page: number) {
        this.loading = true;
        await this.props.fetchPage(page);
        this.loading = false;
    }

    componentDidMount() {
        this.getPage(this.page);
        window.addEventListener('scroll', this.checkScroll);
    }

    componentWillUnmount() {
        window.removeEventListener('scroll', this.checkScroll);
    }

    checkScroll() {
        if (
            !this.loading &&
            window.innerHeight + document.documentElement.scrollTop >= document.documentElement.offsetHeight - 200
        ) {
            this.page += 1;
            this.getPage(this.page);
        }
    }

    render({ items, template }: InfiniteListProps<T>) {
        return items.filter((item) => item).map(template);
    }
}
